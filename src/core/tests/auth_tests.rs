#[cfg(test)]
mod tests {
    use crate::core::auth::{JwtManager, JwtError, JwtConfig};
    use std::{thread::sleep, time::Duration};
    use std::collections::HashMap;
    use jsonwebtoken::Algorithm;

    #[test]
    fn test_secret_generation() {
        let secret = JwtManager::generate_secret();
        assert!(!secret.is_empty());
        assert!(secret.len() >= 32); // Base64 encoded 32 bytes
    }

    #[test]
    fn test_secret_hashing() {
        let secret = "test_secret";
        let hash = JwtManager::hash_secret(secret);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
        
        // Same secret should produce same hash
        let hash2 = JwtManager::hash_secret(secret);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_token_generation() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate token");
        
        assert!(!token.is_empty());
        assert!(token.contains('.')); // JWT contains at least one dot
    }

    #[test]
    fn test_token_with_custom_claims() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        let mut custom_claims = HashMap::new();
        custom_claims.insert("department".to_string(), "engineering".to_string());
        custom_claims.insert("team".to_string(), "backend".to_string());
        
        let token = jwt.generate_token_with_claims("user123", "admin", 3600, Some(custom_claims))
            .expect("Failed to generate token");
        
        let claims = jwt.validate_token(&token)
            .expect("Failed to validate token");
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "admin");
        
        let custom = claims.custom_claims.unwrap();
        assert_eq!(custom.get("department").unwrap(), "engineering");
        assert_eq!(custom.get("team").unwrap(), "backend");
    }

    #[test]
    fn test_token_with_different_algorithms() {
        let secret = JwtManager::generate_secret();
        
        // Test HS384
        let config = JwtConfig {
            algorithm: Algorithm::HS384,
            ..Default::default()
        };
        let jwt = JwtManager::with_config(secret.as_bytes(), config);
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate HS384 token");
        assert!(jwt.validate_token(&token).is_ok());
        
        // Test HS512
        let config = JwtConfig {
            algorithm: Algorithm::HS512,
            ..Default::default()
        };
        let jwt = JwtManager::with_config(secret.as_bytes(), config);
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate HS512 token");
        assert!(jwt.validate_token(&token).is_ok());
    }

    #[test]
    fn test_token_with_audience() {
        let secret = JwtManager::generate_secret();
        let config = JwtConfig {
            aud: Some("test-app".to_string()),
            ..Default::default()
        };
        let jwt = JwtManager::with_config(secret.as_bytes(), config);
        
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate token");
        
        // Should validate with correct audience
        assert!(jwt.validate_token(&token).is_ok());
        
        // Should fail with different audience
        let config = JwtConfig {
            aud: Some("different-app".to_string()),
            ..Default::default()
        };
        let other_jwt = JwtManager::with_config(secret.as_bytes(), config);
        assert!(matches!(other_jwt.validate_token(&token), Err(JwtError::InvalidToken)));
    }

    #[test]
    fn test_token_with_leeway() {
        let secret = JwtManager::generate_secret();
        let config = JwtConfig {
            leeway: 5, // 5 seconds leeway
            ..Default::default()
        };
        let jwt = JwtManager::with_config(secret.as_bytes(), config);
        
        // Generate token that expires in 1 second
        let token = jwt.generate_token("user123", "admin", 1)
            .expect("Failed to generate token");
        
        // Wait for 3 seconds (token would normally be expired, but leeway allows it)
        sleep(Duration::from_secs(3));
        assert!(jwt.validate_token(&token).is_ok());
        
        // Wait for full expiration (beyond leeway)
        sleep(Duration::from_secs(4));
        assert!(matches!(jwt.validate_token(&token), Err(JwtError::TokenExpired)));
    }

    #[test]
    fn test_token_refresh_with_custom_claims() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        let mut original_claims = HashMap::new();
        original_claims.insert("department".to_string(), "engineering".to_string());
        
        let token = jwt.generate_token_with_claims("user123", "admin", 3600, Some(original_claims))
            .expect("Failed to generate token");
        
        let mut new_claims = HashMap::new();
        new_claims.insert("department".to_string(), "devops".to_string());
        
        let new_token = jwt.refresh_token_with_claims(&token, 7200, Some(new_claims))
            .expect("Failed to refresh token");
        
        let claims = jwt.validate_token(&new_token)
            .expect("Failed to validate refreshed token");
        
        let custom = claims.custom_claims.unwrap();
        assert_eq!(custom.get("department").unwrap(), "devops");
    }
} 