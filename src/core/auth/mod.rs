use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use base64::{engine::general_purpose::URL_SAFE, Engine};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: u64,     // Expiration time
    pub iat: u64,     // Issued at
    pub role: String, // User role
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    TokenExpired,
    EncodingError(String),
    DecodingError(String),
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn generate_secret() -> String {
        let mut rng = thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        URL_SAFE.encode(random_bytes)
    }

    pub fn hash_secret(secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn generate_token(&self, user_id: &str, role: &str, expires_in_seconds: u64) -> Result<String, JwtError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + expires_in_seconds,
            iat: now,
            role: role.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| JwtError::EncodingError(e.to_string()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let validation = Validation::default();

        match decode::<Claims>(token, &self.decoding_key, &validation) {
            Ok(token_data) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if token_data.claims.exp < now {
                    Err(JwtError::TokenExpired)
                } else {
                    Ok(token_data.claims)
                }
            }
            Err(_) => Err(JwtError::InvalidToken),
        }
    }

    pub fn refresh_token(&self, old_token: &str, new_expires_in_seconds: u64) -> Result<String, JwtError> {
        let claims = self.validate_token(old_token)?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let new_claims = Claims {
            sub: claims.sub,
            exp: now + new_expires_in_seconds,
            iat: now,
            role: claims.role,
        };

        encode(&Header::default(), &new_claims, &self.encoding_key)
            .map_err(|e| JwtError::EncodingError(e.to_string()))
    }
} 