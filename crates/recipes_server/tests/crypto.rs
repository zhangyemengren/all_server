use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[test]
fn test_argon2() {
    let password = b"123456";
    let salt1 = SaltString::generate(&mut OsRng);
    let salt2 = SaltString::generate(&mut OsRng);
    println!("Salt: {} {}", salt1, salt2);

    // 使用默认参数生成
    let argon2 = Argon2::default();

    // PHC (Password Hashing Competition) string 是一种用于表示密码哈希的标准化格式。
    // 这种格式是为了方便密码哈希的存储、解析和验证，由 Password Hashing Competition (PHC) 提出。
    // 它以字符串的形式将所有密码哈希相关的信息（算法、参数、盐值、哈希值等）集中存储在一起，便于统一管理和操作。
    // ($argon2id$v=19$...)
    let password_hash1 = argon2.hash_password(password, &salt1).unwrap().to_string();
    let password_hash2 = argon2.hash_password(password, &salt2).unwrap().to_string();

    let parsed_hash_1 = PasswordHash::new(&password_hash1).unwrap();
    let parsed_hash_2 = PasswordHash::new(&password_hash2).unwrap();
    assert!(Argon2::default()
        .verify_password(password, &parsed_hash_1)
        .is_ok());
    assert!(Argon2::default()
        .verify_password(password, &parsed_hash_2)
        .is_ok());
}
