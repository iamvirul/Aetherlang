use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use base64::{engine::general_purpose::URL_SAFE, Engine};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,                    // Subject (user ID)
    pub exp: u64,                       // Expiration time
    pub iat: u64,                       // Issued at
    pub role: String,                   // User role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,            // Audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_claims: Option<HashMap<String, String>>, // Custom claims
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    TokenExpired,
    EncodingError(String),
    DecodingError(String),
    InvalidAlgorithm,
}

pub struct JwtConfig {
    pub algorithm: Algorithm,
    pub leeway: u64,              // Leeway in seconds for token expiration
    pub validate_exp: bool,       // Whether to validate expiration
    pub validate_nbf: bool,       // Whether to validate not before claim
    pub aud: Option<String>,      // Expected audience
    pub iss: Option<String>,      // Expected issuer
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::HS256,
            leeway: 0,
            validate_exp: true,
            validate_nbf: false,
            aud: None,
            iss: None,
        }
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    config: JwtConfig,
}

impl JwtManager {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            config: JwtConfig::default(),
        }
    }

    pub fn with_config(secret: &[u8], config: JwtConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            config,
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

    pub fn generate_token_with_claims(
        &self,
        user_id: &str,
        role: &str,
        expires_in_seconds: u64,
        custom_claims: Option<HashMap<String, String>>,
    ) -> Result<String, JwtError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut header = Header::new(self.config.algorithm);
        if let Some(ref iss) = self.config.iss {
            header.kid = Some(iss.clone());
        }

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + expires_in_seconds,
            iat: now,
            role: role.to_string(),
            aud: self.config.aud.clone(),
            custom_claims,
        };

        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| JwtError::EncodingError(e.to_string()))
    }

    pub fn generate_token(&self, user_id: &str, role: &str, expires_in_seconds: u64) -> Result<String, JwtError> {
        self.generate_token_with_claims(user_id, role, expires_in_seconds, None)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(self.config.algorithm);
        validation.leeway = self.config.leeway;
        validation.validate_exp = self.config.validate_exp;
        validation.validate_nbf = self.config.validate_nbf;
        
        if let Some(ref aud) = self.config.aud {
            validation.set_audience(&[aud]);
        }
        if let Some(ref iss) = self.config.iss {
            validation.set_issuer(&[iss]);
        }

        match decode::<Claims>(token, &self.decoding_key, &validation) {
            Ok(token_data) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if token_data.claims.exp + self.config.leeway < now {
                    Err(JwtError::TokenExpired)
                } else {
                    Ok(token_data.claims)
                }
            }
            Err(_) => Err(JwtError::InvalidToken),
        }
    }

    pub fn refresh_token_with_claims(
        &self,
        old_token: &str,
        new_expires_in_seconds: u64,
        custom_claims: Option<HashMap<String, String>>,
    ) -> Result<String, JwtError> {
        let claims = self.validate_token(old_token)?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut header = Header::new(self.config.algorithm);
        if let Some(ref iss) = self.config.iss {
            header.kid = Some(iss.clone());
        }

        let new_claims = Claims {
            sub: claims.sub,
            exp: now + new_expires_in_seconds,
            iat: now,
            role: claims.role,
            aud: self.config.aud.clone(),
            custom_claims: custom_claims.or(claims.custom_claims),
        };

        encode(&header, &new_claims, &self.encoding_key)
            .map_err(|e| JwtError::EncodingError(e.to_string()))
    }

    pub fn refresh_token(&self, old_token: &str, new_expires_in_seconds: u64) -> Result<String, JwtError> {
        self.refresh_token_with_claims(old_token, new_expires_in_seconds, None)
    }
} 