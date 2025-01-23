#![cfg(feature = "validate_derive")]

use utils::Validate;
use utils::Validator;

#[derive(Validate)]
struct TestEmail {
    #[validate(email)]
    email: String,
}

#[derive(Validate)]
struct TestPassword {
    #[validate(password)]
    password: String,
}

#[derive(Validate)]
#[validate]
struct TestCustomValidation {
    #[validate(email)]
    _email: String,
}

#[test]
fn test_email_validation() {
    let valid = TestEmail {
        email: "user@example.com".to_string(),
    };
    let invalid_format = TestEmail {
        email: "not-an-email".to_string(),
    };
    let empty = TestEmail {
        email: "".to_string(),
    };

    assert!(valid.validate().is_ok());
    assert!(invalid_format.validate().is_err());
    assert!(empty.validate().is_err());
}

#[test]
fn test_password_validation() {
    let valid = TestPassword {
        password: "Password123@".to_string(),
    };
    let weak = TestPassword {
        password: "12345678".to_string(),
    };
    let no_special = TestPassword {
        password: "Password123".to_string(),
    };
    let no_uppercase = TestPassword {
        password: "password123@".to_string(),
    };
    let no_lowercase = TestPassword {
        password: "PASSWORD123@".to_string(),
    };
    let no_digit = TestPassword {
        password: "Password@".to_string(),
    };
    let too_short = TestPassword {
        password: "Pw1@".to_string(),
    };
    let empty = TestPassword {
        password: "".to_string(),
    };

    assert!(valid.validate().is_ok());
    assert!(weak.validate().is_err());
    assert!(no_special.validate().is_err());
    assert!(no_uppercase.validate().is_err());
    assert!(no_lowercase.validate().is_err());
    assert!(no_digit.validate().is_err());
    assert!(too_short.validate().is_err());
    assert!(empty.validate().is_err());
}

#[test]
fn test_custom_validation() {
    let test = TestCustomValidation {
        _email: "not-validated@example.com".to_string(),
    };

    assert!(test.validate(|_| Ok(())).is_ok());
    assert!(test
        .validate(|_| Err("Custom validation failed".to_string()))
        .is_err());
}

#[test]
fn test_validator_directly() {
    assert!(Validator::validate_email("user@example.com"));
    assert!(!Validator::validate_email("not-an-email"));
    assert!(!Validator::validate_email(""));

    assert!(Validator::validate_password("Password123@"));
    assert!(!Validator::validate_password("weak"));
    assert!(!Validator::validate_password(""));
}
