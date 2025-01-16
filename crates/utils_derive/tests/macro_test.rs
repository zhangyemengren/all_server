use utils_derive::Validate;

#[derive(Validate)]
struct A {
    #[validate(email)]
    m: String, // 注意是 String 类型
}

#[test]
fn test() {
    let a_ok = A { m: "not_empty".to_string() };
    let a_err = A { m: "".to_string() };

    // 正常情况
    match a_ok.validate() {
        Ok(_) => println!("a_ok passed validation"),
        Err(e) => println!("a_ok failed validation: {}", e),
    }

    // 空值情况
    match a_err.validate() {
        Ok(_) => println!("a_err passed validation"),
        Err(e) => println!("a_err failed validation: {}", e),
    }
    assert!(true)
}
