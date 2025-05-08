#[cfg(test)]
mod tests {
    use crate::core::auth::{JwtManager, JwtError};
    use std::{thread::sleep, time::Duration};

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
    fn test_token_validation() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate token");
        
        let claims = jwt.validate_token(&token)
            .expect("Failed to validate token");
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "admin");
    }

    #[test]
    fn test_token_expiration() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        // Generate token that expires in 1 second
        let token = jwt.generate_token("user123", "admin", 1)
            .expect("Failed to generate token");
        
        // Token should be valid initially
        assert!(jwt.validate_token(&token).is_ok());
        
        // Wait for token to expire
        sleep(Duration::from_secs(2));
        
        // Token should be expired now
        match jwt.validate_token(&token) {
            Err(JwtError::TokenExpired) => (),
            _ => panic!("Token should be expired"),
        }
    }

    #[test]
    fn test_invalid_token() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        // Try to validate invalid token
        let result = jwt.validate_token("invalid.token.here");
        assert!(matches!(result, Err(JwtError::InvalidToken)));
        
        // Try to validate token with wrong secret
        let other_jwt = JwtManager::new("different_secret".as_bytes());
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate token");
        
        let result = other_jwt.validate_token(&token);
        assert!(matches!(result, Err(JwtError::InvalidToken)));
    }

    #[test]
    fn test_token_refresh() {
        let secret = JwtManager::generate_secret();
        let jwt = JwtManager::new(secret.as_bytes());
        
        // Generate original token
        let token = jwt.generate_token("user123", "admin", 3600)
            .expect("Failed to generate token");
        
        // Refresh token
        let new_token = jwt.refresh_token(&token, 7200)
            .expect("Failed to refresh token");
        
        // Validate new token
        let claims = jwt.validate_token(&new_token)
            .expect("Failed to validate refreshed token");
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "admin");
        
        // Original claims
        let original_claims = jwt.validate_token(&token)
            .expect("Failed to validate original token");
        
        // New token should have later expiration
        assert!(claims.exp > original_claims.exp);
    }
} 