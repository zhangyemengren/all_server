#[cfg(feature = "validate_derive")]
pub use utils_derive::Validate;

// 重导出 regex，这样使用宏的 crate 就不需要直接依赖 regex 了
#[cfg(feature = "validate_derive")]
pub use regex;

#[cfg(feature = "validate_derive")]
mod validator {
    use regex::Regex;
    use std::sync::LazyLock;

    static EMAIL_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

    static PASSWORD_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap());

    pub struct Validator;

    impl Validator {
        pub fn validate_email(email: &str) -> bool {
            EMAIL_REGEX.is_match(email)
        }

        pub fn validate_password(password: &str) -> bool {
            if !PASSWORD_REGEX.is_match(password) {
                return false;
            }
            // 检查是否包含小写字母
            let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
            // 检查是否包含大写字母
            let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
            // 检查是否包含数字
            let has_digit = password.chars().any(|c| c.is_ascii_digit());
            // 检查是否包含特殊字符
            let has_special = password.chars().any(|c| "@$!%*?&".contains(c));

            has_lowercase && has_uppercase && has_digit && has_special
        }
    }
}

#[cfg(feature = "validate_derive")]
pub use validator::Validator;
