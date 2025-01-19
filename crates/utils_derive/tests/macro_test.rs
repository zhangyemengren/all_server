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
#[derive(Validate)]
struct C {
    #[validate(password)]
    p: String, // 注意是 String 类型
}

#[test]
fn test_email_filed() {
    let a_ok = A {
        m: "123@qq.com".to_string(),
    };
    let a_err = A { m: "".to_string() };
    let a_err2 = A {
        m: "123qq.com".to_string(),
    };

    assert_eq!(a_ok.validate().is_ok(), true);
    assert_eq!(a_err.validate().is_err(), true);
    assert_eq!(a_err2.validate().is_err(), true);
}
#[test]
fn test_password_filed() {
    let weak_password = C {
        p: "12345678".to_string(),
    };
    let without_special_char = C {
        p: "12345678Qxx".to_string(),
    };
    let without_uppercase = C {
        p: "12345678@xx".to_string(),
    };
    let without_lowercase = C {
        p: "12345678@XX".to_string(),
    };
    let without_digit = C {
        p: "xxxxxc@Qxx".to_string(),
    };
    let less_than_8_char = C {
        p: "123@Qx".to_string(),
    };
    let strong_password = C {
        p: "12345678@Qxx".to_string(),
    };
    let empty_password = C { p: "".to_string() };

    assert_eq!(strong_password.validate().is_ok(), true);
    assert_eq!(weak_password.validate().is_err(), true);
    assert_eq!(empty_password.validate().is_err(), true);
    assert_eq!(without_special_char.validate().is_err(), true);
    assert_eq!(without_uppercase.validate().is_err(), true);
    assert_eq!(without_lowercase.validate().is_err(), true);
    assert_eq!(without_digit.validate().is_err(), true);
    assert_eq!(less_than_8_char.validate().is_err(), true);
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
