use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields};

/// 生成自定义Derive宏
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let ast = parse_macro_input!(input as DeriveInput);
    // 获取结构体名
    let struct_ident = &ast.ident;
    // 检查结构体上是否有 #[validate(...)] 属性
    let has_struct_validate = has_struct_level_validate(&ast.attrs);

    // 根据结构体上是否有 #[validate(...)] 属性，生成不同的代码
    let expanded = if has_struct_validate {
        // 如果结构体上有 #[validate(...)] 属性，则生成一个可传入Fn的校验方法，忽略字段校验
        quote! {
            impl #struct_ident {
                pub fn validate<F>(&self, f: F) -> Result<(), String>
                where
                    F: Fn(&Self) -> Result<(), String>
                {
                    f(self)
                }
            }
        }
    } else {
        // 如果结构体上没有 #[validate(...)] 属性，则生成一个校验方法，对每个字段进行校验
        let mut field_checks = proc_macro2::TokenStream::new();

        // 检查是否为结构体
        if let Data::Struct(data_struct) = &ast.data {
            // 检查是否为命名结构体（非元组，单元结构体）
            if let Fields::Named(fields_named) = &data_struct.fields {
                // 遍历字段
                fields_named.named.iter()
                    .filter_map(|field| field.ident.as_ref().map(|field_ident| (field, field_ident))) // 过滤掉没有字段名的字段
                    .for_each(|(field, field_ident)| {
                        // 遍历字段上的属性，查找 #[validate(...)]
                        field.attrs.iter()
                            .filter_map(get_validate_type_from_attr) // 过滤掉没有validator_type
                            .for_each(|validator_type| {
                                // 根据 validator_type 生成不同的校验逻辑
                                let check_code = match validator_type.as_str() {
                                    // 邮箱校验
                                    "email" => quote! {
                                        // 因为这个代码是在 proc macro 中生成的，会在使用这个宏的 crate 中展开。我们需要使用完整的路径。
                                        let is_valid = ::utils::Validator::validate_email(&self.#field_ident);
                                        if !is_valid {
                                            errors.push(format!("`{}` is not a valid email address", stringify!(#field_ident)));
                                        }
                                    },
                                    // 密码校验
                                    "password" => quote! {
                                        let is_valid = ::utils::Validator::validate_password(&self.#field_ident);
                                        if !is_valid {
                                            errors.push(format!("`{}` is not a valid password", stringify!(#field_ident)));
                                        }
                                    },
                                    _ => quote! {
                                        errors.push(format!("Unknown validator `{}` for `{}`", #validator_type, stringify!(#field_ident)));
                                    },
                                };
                                // 将检查代码插入到 field_checks 中
                                field_checks.extend(check_code);
                            })
                    });
            }
        }

        quote! {
            impl #struct_ident {
                pub fn validate(&self) -> Result<(), String> {
                    let mut errors = Vec::new();

                    // 插入对每个字段的检查逻辑
                    #field_checks

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(errors.join("; "))
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// 解析字段上的属性，获取校验类型
fn get_validate_type_from_attr(attr: &Attribute) -> Option<String> {
    // 确认属性名是否是 "validate"
    if !attr.path().is_ident("validate") {
        return None;
    }

    let mut result = None;

    // 解析属性内部内容
    let parse_result = attr.parse_nested_meta(|meta| {
        // 若 meta 为 email，则记录下来
        match meta.path.get_ident() {
            Some(ident) if ident == "email" => result = Some("email".to_string()),
            Some(ident) if ident == "password" => result = Some("password".to_string()),
            _ => (),
        }
        // 其他情况可继续扩展
        Ok(())
    });

    // 若解析失败，则直接返回 None
    if parse_result.is_err() {
        return None;
    }

    result
}

/// 检查结构体上是否有 #[validate(...)] 属性
fn has_struct_level_validate(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        // 检查属性名是否为 "validate"
        attr.path().is_ident("validate")
    })
}
