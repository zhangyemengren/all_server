use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, Attribute,
};

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // 1. 将传入的 TokenStream 解析为 syn 的数据结构
    let ast = parse_macro_input!(input as DeriveInput);

    // 2. 获取被标注类型（结构体）的名称
    let struct_name = &ast.ident;

    // 准备一个容器，用于存储要生成的“检查逻辑”
    let mut field_checks = proc_macro2::TokenStream::new();


    // 3. 只处理 struct，本例不考虑 enum/union
    if let Data::Struct(data_struct) = &ast.data {
        // 4. 获取结构体所有字段
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    // 获取字段名称（比如 m）
                    let field_ident = match &field.ident {
                        Some(ident) => ident,
                        None => continue,
                    };

                    // 5. 遍历字段上的属性，查找 #[validate(...)]
                    for attr in &field.attrs {
                        // 尝试解析属性
                        if let Some(validator_type) = get_validate_type_from_attr(attr) {
                            // 判断 validator_type 是否为 "email"
                            if validator_type == "email" {
                                // 这里仅对空值进行检查
                                let check_code = quote! {
                                    if self.#field_ident.is_empty() {
                                        errors.push(format!("`{}` cannot be empty", stringify!(#field_ident)));
                                    }
                                };
                                // 把这段检查代码，拼进 field_checks 里
                                field_checks.extend(check_code);
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }

    // 6. 生成最终的实现代码：
    //    impl StructName { fn validate(&self) -> Result<(), String> { ... } }
    let expanded = quote! {
        impl #struct_name {
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
    };

    // 7. 转换为 TokenStream 返回给编译器
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
