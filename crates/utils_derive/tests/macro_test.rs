use utils_derive::Validate;

#[derive(Validate)]
struct A {
    #[validate(email)]
    m: String, // 注意是 String 类型
}
#[derive(Validate)]
#[validate]
struct B {
    #[validate(email)]
    _m: String, // 注意是 String 类型
}

#[test]
fn test_filed() {
    let a_ok = A {
        m: "123@qq.com".to_string(),
    };
    let a_err = A { m: "".to_string() };

    assert_eq!(a_ok.validate().is_ok(), true);
    assert_eq!(a_err.validate().is_err(), true);
}
#[test]
fn test_struct() {
    let b_ok = B {
        _m: "123@qq.com".to_string(),
    };
    let b_empty = B { _m: "".to_string() };

    assert_eq!(b_ok.validate(|_| Ok(())).is_ok(), true);
    assert_eq!(b_empty.validate(|_| Ok(())).is_ok(), true);
}
