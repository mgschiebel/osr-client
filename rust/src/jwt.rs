use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Claims struct for JWT payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub exp: u64,    // expiry timestamp
    pub game_server: Option<String>,
}

/// Create a JWT token with HS256 algorithm.
pub fn create_jwt(
    username: &str,
    secret: &[u8],
    ttl_seconds: u64,
    game_server: Option<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = Claims {
        sub: username.to_string(),
        exp: now + ttl_seconds,
        game_server,
    };
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

/// Decode and validate a JWT token.
pub fn validate_jwt(
    token: &str,
    secret: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(decoded.claims)
}

/// Check if a JWT token is expired (returns true if expired).
pub fn is_expired(token: &str, secret: &[u8]) -> bool {
    match validate_jwt(token, secret) {
        Ok(claims) => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            claims.exp < now
        }
        Err(_) => true, // Invalid token is treated as expired
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &[u8] = b"test-secret-key-12345";

    #[test]
    fn test_create_and_validate_jwt() {
        let token = create_jwt("testuser", TEST_SECRET, 3600, None).unwrap();
        let claims = validate_jwt(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.sub, "testuser");
        assert!(claims.exp > 0);
    }

    #[test]
    fn test_create_jwt_with_game_server() {
        let game_server = "wss://game.example.com".to_string();
        let token = create_jwt("testuser", TEST_SECRET, 3600, Some(game_server.clone())).unwrap();
        let claims = validate_jwt(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.game_server, Some(game_server));
    }

    #[test]
    fn test_invalid_token_returns_error() {
        let result = validate_jwt("invalid-token", TEST_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret_returns_error() {
        let token = create_jwt("testuser", TEST_SECRET, 3600, None).unwrap();
        let wrong_secret = b"wrong-secret";
        let result = validate_jwt(&token, wrong_secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        // Create a token with exp in the past
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let claims = Claims {
            sub: "testuser".to_string(),
            exp: now - 3600, // expired 1 hour ago
            game_server: None,
        };
        let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(TEST_SECRET)).unwrap();
        assert!(is_expired(&token, TEST_SECRET));
    }

    #[test]
    fn test_valid_token_not_expired() {
        let token = create_jwt("testuser", TEST_SECRET, 3600, None).unwrap();
        assert!(!is_expired(&token, TEST_SECRET));
    }
}
