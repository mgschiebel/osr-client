use crate::jwt::{create_jwt, Claims};
use crate::shared::{AuthError, AuthRequest, AuthResponse, JwtToken};
use crate::logging::log_error;
use jsonwebtoken::Algorithm;

/// Mock auth server configuration.
#[derive(Debug, Clone)]
pub struct AuthServerConfig {
    pub port: u16,
    pub game_server_addr: String,
    pub jwt_secret: String,
    pub jwt_ttl_seconds: u64,
}

impl Default for AuthServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            game_server_addr: "wss://localhost:9090".to_string(),
            jwt_secret: "mock-secret-key-for-testing".to_string(),
            jwt_ttl_seconds: 3600,
        }
    }
}

/// Handle a single authentication request.
pub fn handle_auth_request(
    config: &AuthServerConfig,
    request: &AuthRequest,
) -> AuthResponse {
    // Mock validation: accept any non-empty username/password
    if request.username.is_empty() || request.password.is_empty() {
        log_error(
            "auth_server",
            "Invalid credentials: empty username or password",
            Some(serde_json::json!({"username": request.username})),
        );
        return AuthResponse {
            success: false,
            token: None,
            game_server: None,
            error: Some(AuthError::InvalidCredentials),
        };
    }

    // Create JWT token
    let token_string = match create_jwt(
        &request.username,
        config.jwt_secret.as_bytes(),
        config.jwt_ttl_seconds,
        Some(config.game_server_addr.clone()),
    ) {
        Ok(token) => token,
        Err(e) => {
            log_error(
                "auth_server",
                "JWT creation failed",
                Some(serde_json::json!({"error": format!("{:?}", e)})),
            );
            return AuthResponse {
                success: false,
                token: None,
                game_server: None,
                error: Some(AuthError::ServerError),
            };
        }
    };

    // Get expiry from the token claims
    let exp = jsonwebtoken::decode::<Claims>(
        &token_string,
        &jsonwebtoken::DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::new(Algorithm::HS256),
    )
    .map(|d| d.claims.exp)
    .unwrap_or(0);

    AuthResponse {
        success: true,
        token: Some(JwtToken {
            token: token_string,
            expires_at: exp,
        }),
        game_server: Some(config.game_server_addr.clone()),
        error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_valid_auth_request() {
        let config = AuthServerConfig::default();
        let request = AuthRequest {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        };

        let response = handle_auth_request(&config, &request);
        assert!(response.success);
        assert!(response.token.is_some());
        assert_eq!(response.game_server, Some(config.game_server_addr.clone()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_handle_auth_request_empty_username() {
        let config = AuthServerConfig::default();
        let request = AuthRequest {
            username: "".to_string(),
            password: "testpass".to_string(),
        };

        let response = handle_auth_request(&config, &request);
        assert!(!response.success);
        assert!(response.token.is_none());
        assert_eq!(response.error, Some(AuthError::InvalidCredentials));
    }

    #[test]
    fn test_handle_auth_request_empty_password() {
        let config = AuthServerConfig::default();
        let request = AuthRequest {
            username: "testuser".to_string(),
            password: "".to_string(),
        };

        let response = handle_auth_request(&config, &request);
        assert!(!response.success);
        assert!(response.token.is_none());
        assert_eq!(response.error, Some(AuthError::InvalidCredentials));
    }

    #[test]
    fn test_jwt_token_in_response_is_valid() {
        let config = AuthServerConfig::default();
        let request = AuthRequest {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        };

        let response = handle_auth_request(&config, &request);
        assert!(response.success);
        let token = response.token.unwrap();

        // Verify the JWT can be decoded
        let claims = crate::jwt::validate_jwt(&token.token, config.jwt_secret.as_bytes()).unwrap();
        assert_eq!(claims.sub, "testuser");
        assert!(claims.exp > 0);
    }
}
