use std::fmt::Debug;
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

pub struct Crypto;

impl Crypto {
    const SECRET: &'static [u8] = b"secret";
    pub fn hash_password(password: &[u8]) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let password_hash = argon
            .hash_password(password, &salt)
            .map_err(|e| anyhow::anyhow!(e))?
            .to_string();
        Ok(password_hash)
    }
    pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool> {
        let argon = Argon2::default();
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| anyhow!(e))?;
        Ok(argon.verify_password(password, &parsed_hash).is_ok())
    }
    pub fn encode_token<T>(claims: T) -> Result<String>
    where
        T: Debug + Serialize,
    {
        let header = Header::default();
        let key = Self::SECRET;
        let token = encode(&header, &claims, &EncodingKey::from_secret(key))?;
        Ok(token)
    }
    pub fn decode_token<T>(token: &str) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let key = Self::SECRET;
        let token_data = decode::<T>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_hash_password_success() -> Result<()> {
        let password = b"my_secure_password";
        let hash = Crypto::hash_password(password)?;

        // 确保哈希字符串不为空
        assert!(!hash.is_empty());

        // 确保哈希字符串包含预期的前缀（argon2 默认格式）
        assert!(hash.starts_with("$argon2"));

        Ok(())
    }

    #[test]
    fn test_verify_password_success() -> Result<()> {
        let password = b"another_secure_password";
        let hash = Crypto::hash_password(password)?;

        let is_valid = Crypto::verify_password(password, &hash)?;
        assert!(is_valid, "正确的密码应该验证成功");

        Ok(())
    }

    #[test]
    fn test_verify_password_failure() -> Result<()> {
        let password = b"password123";
        let wrong_password = b"wrong_password";
        let hash = Crypto::hash_password(password)?;

        let is_valid = Crypto::verify_password(wrong_password, &hash)?;
        assert!(!is_valid, "错误的密码应该验证失败");

        Ok(())
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = b"password123";
        let invalid_hash = "invalid_hash_string";

        let result = Crypto::verify_password(password, invalid_hash);
        assert!(result.is_err(), "提供无效的哈希字符串时应该返回错误");
    }

    #[test]
    fn test_hash_password_uniqueness() -> Result<()> {
        let password = b"unique_password";
        let hash1 = Crypto::hash_password(password)?;
        let hash2 = Crypto::hash_password(password)?;

        // 即使密码相同，由于使用了不同的盐，哈希应该不同
        assert_ne!(hash1, hash2, "使用不同盐时，哈希应该不同");
        let is_valid1 = Crypto::verify_password(password, &hash1)?;
        let is_valid2 = Crypto::verify_password(password, &hash2)?;
        // 验证都应该通过
        assert!(is_valid1);
        assert!(is_valid2);

        Ok(())
    }
    #[test]
    fn test_token_success() -> Result<()> {
        use jiff::Timestamp;
        use std::time::Duration;

        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }

        let exp = Timestamp::now() + Duration::from_secs(60);
        let claims = Claims {
            sub: "user_id".to_string(),
            exp: exp.as_second() as usize,
        };

        let token = Crypto::encode_token(&claims)?;
        println!("token: {}", token);
        assert!(!token.is_empty(), "生成的 token 不应该为空");

        let decoded_claims: Claims = Crypto::decode_token(&token)?;
        assert_eq!(claims.sub, decoded_claims.sub);
        print!("decoded_claims: {:?}", decoded_claims);

        Ok(())
    }

    #[test]
    fn test_token_expired() -> Result<()> {
        use jiff::Timestamp;
        use std::time::Duration;

        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: u64,
        }

        // jwt库默认leeway偏差为60秒 实际需要超时60秒以上才过期
        let exp = Timestamp::now() - Duration::from_secs(61);
        let claims = Claims {
            sub: "user_id".to_string(),
            exp: exp.as_second() as u64,
        };

        let token = Crypto::encode_token(&claims)?;
        let result = Crypto::decode_token::<Claims>(&token);
        assert!(result.is_err(), "过期的 token 应该解码失败");

        Ok(())
    }
}
