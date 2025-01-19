use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields};


#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;
    let has_struct_validate = has_struct_level_validate(&ast.attrs);
    

    let expanded = if has_struct_validate {
        // 仅生成可传入Fn的校验方法，忽略字段校验
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
        // 原有字段校验逻辑
        // 准备一个容器，用于存储要生成的"检查逻辑"
        let mut field_checks = proc_macro2::TokenStream::new();

        if let Data::Struct(data_struct) = &ast.data {
            // 4. 获取结构体所有字段
            if let Fields::Named(fields_named) = &data_struct.fields {
                fields_named.named.iter()
                    .filter_map(|field| field.ident.as_ref().map(|field_ident| (field, field_ident))) // 过滤掉没有字段名的字段
                    .for_each(|(field, field_ident)| {
                        // 5. 遍历字段上的属性，查找 #[validate(...)]
                        field.attrs.iter()
                            .filter_map(|attr| get_validate_type_from_attr(attr)) // 过滤掉没有validator_type
                            .for_each(|validator_type| {
                                // 根据 validator_type 生成不同的校验逻辑
                                let check_code = match validator_type.as_str() {
                                    "email" => quote! {
                                        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
                                        if !email_regex.is_match(&self.#field_ident) {
                                            errors.push(format!("`{}` is not a valid email address", stringify!(#field_ident)));
                                        }
                                    },
                                    _ => quote! {
                                        errors.push(format!("Unknown validator `{}` for `{}`", #validator_type, stringify!(#field_ident)));
                                    },
                                };
                                // 拼接检查代码
                                field_checks.extend(check_code);
                            })
                    });
            }
        }

        quote! {
            impl #struct_ident {
                pub fn validate(&self) -> Result<(), String> {
                    let mut errors = Vec::new();

                    // 这里直接插入对每个字段的检查逻辑
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

/// 解析 #[validate(...)] 属性，若为 `#[validate(email)]`，则返回 Some("email")
fn get_validate_type_from_attr(attr: &Attribute) -> Option<String> {
    // 1. 确认属性名是否是 "validate"
    if !attr.path().is_ident("validate") {
        return None;
    }

    // 2. 使用 parse_nested_meta 解析属性内部内容
    //    例如: #[validate(email)], #[validate(something_else)]
    let mut result = None;
    let parse_result = attr.parse_nested_meta(|meta| {
        // 若 meta 为 email，则记录下来
        if meta.path.is_ident("email") {
            result = Some("email".to_string());
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

fn has_struct_level_validate(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        // 检查属性名是否为 "validate"
        attr.path().is_ident("validate")
    })
}
