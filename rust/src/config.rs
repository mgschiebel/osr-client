use serde::Deserialize;
use std::fs;

/// Client configuration loaded from client.toml.
#[derive(Debug, Deserialize, Default)]
pub struct ClientConfig {
    pub auth_server: Option<String>,
    pub jwt_secret: Option<String>,
}

impl ClientConfig {
    /// Load configuration from client.toml in the specified directory.
    pub fn load_from_dir(dir: &str) -> Result<Self, String> {
        let path = format!("{}/client.toml", dir);
        match fs::read_to_string(&path) {
            Ok(contents) => {
                toml::from_str(&contents)
                    .map_err(|e| format!("Failed to parse client.toml: {}", e))
            }
            Err(_) => {
                // File doesn't exist, return defaults
                Ok(Self::default())
            }
        }
    }

    /// Get the auth server URL, with fallback to default.
    pub fn auth_server_url(&self) -> String {
        self.auth_server
            .clone()
            .unwrap_or_else(|| "wss://localhost:8080".to_string())
    }

    /// Get the JWT secret, with fallback to default.
    pub fn jwt_secret(&self) -> String {
        self.jwt_secret
            .clone()
            .unwrap_or_else(|| "mock-secret-key-for-testing".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.auth_server_url(), "wss://localhost:8080");
        assert_eq!(config.jwt_secret(), "mock-secret-key-for-testing");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let config = ClientConfig::load_from_dir("/nonexistent/path").unwrap();
        assert_eq!(config.auth_server_url(), "wss://localhost:8080");
    }
}
