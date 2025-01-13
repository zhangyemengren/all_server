use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn derive_my_macro(input: TokenStream) -> TokenStream {
    // 1. 将传入的 TokenStream 解析为 syn 的数据结构
    let ast = parse_macro_input!(input as DeriveInput);

    // 2. 获取类型名称
    let name = &ast.ident;

    // 3. 使用 quote! 来生成目标实现的 TokenStream
    let expanded = quote! {
        impl #name {
            pub fn hello() -> String{
                let msg = format!("Hello from {}", stringify!(#name));
                println!("{}", msg);
                msg
            }
        }
    };

    // 4. 将生成的语法树转换回 TokenStream 返回给编译器
    TokenStream::from(expanded)
}
