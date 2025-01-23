use utils::Validate;

#[derive(Validate)]
pub struct User {
    #[validate(email)]
    email: String,
    #[validate(password)]
    password: String,
}

pub async fn validate_user() -> &'static str {
    let user = User {
        email: "test@example.com".to_string(),
        password: "password12Q!".to_string(),
    };
    assert!(user.validate().is_ok());
    "User validated"
}
