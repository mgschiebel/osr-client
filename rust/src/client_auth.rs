use crate::jwt::is_expired;
use crate::shared::{AuthError, AuthRequest, AuthResponse};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::runtime::Runtime;

/// Auth client configuration, loaded from client.toml.
#[derive(Clone, Debug)]
pub struct AuthClientConfig {
    pub auth_server_url: String,
    pub jwt_secret: String,
}

impl AuthClientConfig {
    /// Load from client.toml in the specified directory.
    pub fn load(dir: &str) -> Self {
        match crate::config::ClientConfig::load_from_dir(dir) {
            Ok(config) => Self {
                auth_server_url: config.auth_server_url(),
                jwt_secret: config.jwt_secret(),
            },
            Err(_) => Self::default(),
        }
    }
}

impl Default for AuthClientConfig {
    fn default() -> Self {
        Self {
            auth_server_url: "wss://localhost:8080".to_string(),
            jwt_secret: "mock-secret-key-for-testing".to_string(),
        }
    }
}

/// Auth result for returning to Godot.
pub struct AuthResult {
    pub success: bool,
    pub token: Option<String>,
    pub game_server: Option<String>,
    pub error: Option<AuthError>,
}

/// Authenticate with the auth server (blocking, spawns tokio runtime).
pub fn authenticate(
    config: &AuthClientConfig,
    username: &str,
    password: &str,
) -> Result<AuthResult, String> {
    let rt = Runtime::new()
        .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

    rt.block_on(async {
        // Connect to auth server
        let url = url::Url::parse(&config.auth_server_url)
            .map_err(|e| format!("Invalid URL: {}", e))?;

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| format!("WebSocket connection failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        // Send AuthRequest
        let request = AuthRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        use futures_util::SinkExt;
        write
            .send(Message::Text(request_json))
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        // Wait for AuthResponse
        use futures_util::StreamExt;
        let response = match read.next().await {
            Some(Ok(Message::Text(text))) => {
                serde_json::from_str::<AuthResponse>(&text)
                    .map_err(|e| format!("Failed to parse response: {}", e))?
            }
            Some(Ok(Message::Close(_))) => {
                return Err("Connection closed before response".to_string());
            }
            Some(Err(e)) => {
                return Err(format!("WebSocket error: {}", e));
            }
            None => {
                return Err("Connection closed before response".to_string());
            }
            _ => {
                return Err("Unexpected message type".to_string());
            }
        };

        if response.success {
            Ok(AuthResult {
                success: true,
                token: response.token.map(|t| t.token),
                game_server: response.game_server,
                error: None,
            })
        } else {
            Ok(AuthResult {
                success: false,
                token: None,
                game_server: None,
                error: response.error,
            })
        }
    })
}

/// Check if a JWT token is expired.
pub fn check_token_expired(token: &str, secret: &str) -> bool {
    is_expired(token, secret.as_bytes())
}
