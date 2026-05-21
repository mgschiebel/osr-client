use serde::{Deserialize, Serialize};

/// Authentication request sent from client to server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

/// JWT token wrapper with expiry info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JwtToken {
    pub token: String,
    pub expires_at: u64, // Unix timestamp
}

/// Authentication response sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<JwtToken>,
    pub game_server: Option<String>,
    pub error: Option<AuthError>,
}

/// Typed authentication errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthError {
    ConnectionFailed,
    Timeout,
    InvalidCredentials,
    ServerError,
    TokenExpired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_request_serialization() {
        let req = AuthRequest {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: AuthRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_auth_response_success_serialization() {
        let resp = AuthResponse {
            success: true,
            token: Some(JwtToken {
                token: "fake-jwt-token".to_string(),
                expires_at: 1234567890,
            }),
            game_server: Some("wss://game.example.com".to_string()),
            error: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: AuthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_auth_response_failure_serialization() {
        let resp = AuthResponse {
            success: false,
            token: None,
            game_server: None,
            error: Some(AuthError::InvalidCredentials),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: AuthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_auth_error_all_variants() {
        let errors = vec![
            AuthError::ConnectionFailed,
            AuthError::Timeout,
            AuthError::InvalidCredentials,
            AuthError::ServerError,
            AuthError::TokenExpired,
        ];
        for error in errors {
            let json = serde_json::to_string(&error).unwrap();
            let deserialized: AuthError = serde_json::from_str(&json).unwrap();
            assert_eq!(error, deserialized);
        }
    }
}
