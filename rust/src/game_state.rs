use godot::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// Auth result for communicating between threads.
enum AuthResult {
    Success { token: String, game_server: String },
    Failure { error: String },
}

/// Rust autoload singleton managing scene transitions and auth.
#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct GameState {
    base: Base<Node>,
    current_scene: GString,
    auth_token: GString,
    game_server: GString,
    auth_result_rx: Option<Receiver<AuthResult>>,
}

#[godot_api]
impl INode for GameState {
    fn ready(&mut self) {
        self.current_scene = GString::from("LoadingScreen");
        godot_print!("GameState ready, current scene: {}", self.current_scene);
    }

    fn process(&mut self, _delta: f64) {
        // Check for auth result
        if let Some(rx) = &self.auth_result_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    AuthResult::Success { token, game_server } => {
                        self.auth_token = GString::from(&token);
                        self.game_server = GString::from(&game_server);
                        self.base_mut()
                            .emit_signal("auth_succeeded", &[token.to_variant(), game_server.to_variant()]);
                    }
                    AuthResult::Failure { error } => {
                        self.base_mut()
                            .emit_signal("auth_failed", &[GString::from(&error).to_variant()]);
                    }
                }
            }
        }
    }
}

#[godot_api]
impl GameState {
    #[signal]
    fn auth_succeeded(token: GString, game_server: GString);

    #[signal]
    fn auth_failed(error: GString);

    #[func]
    fn get_current_scene(&self) -> GString {
        self.current_scene.clone()
    }

    #[func]
    fn get_auth_token(&self) -> GString {
        self.auth_token.clone()
    }

    #[func]
    fn get_game_server(&self) -> GString {
        self.game_server.clone()
    }

    #[func]
    fn transition_to(&mut self, scene_name: GString) {
        let valid_transitions = [
            ("LoadingScreen", "AuthScreen"),
            ("AuthScreen", "CharacterSelectScreen"),
            ("CharacterSelectScreen", "EmptyRoom"),
        ];

        let scene_str = scene_name.to_string();
        let current_str = self.current_scene.to_string();

        let valid = valid_transitions
            .iter()
            .any(|(from, to)| current_str == *from && scene_str == *to);

        if valid {
            self.current_scene = scene_name.clone();
            let tree = self.base().get_tree();
            if let Some(mut tree) = tree {
                let path = match scene_str.as_str() {
                    "AuthScreen" => "res://scenes/auth/AuthScreen.tscn",
                    "CharacterSelectScreen" => "res://scenes/character_select/CharacterSelectScreen.tscn",
                    "EmptyRoom" => "res://scenes/world/EmptyRoom.tscn",
                    _ => return,
                };
                tree.change_scene_to_file(path);
            }
        } else {
            godot_error!("Invalid transition: {} -> {}", current_str, scene_str);
        }
    }

    #[func]
    fn transition_to_auth(&mut self) {
        self.transition_to(GString::from("AuthScreen"));
    }

    #[func]
    fn transition_to_character_select(&mut self) {
        self.transition_to(GString::from("CharacterSelectScreen"));
    }

    #[func]
    fn transition_to_world(&mut self) {
        self.transition_to(GString::from("EmptyRoom"));
    }

    /// Authenticate with the auth server. Emits auth_succeeded or auth_failed signal.
    #[func]
    fn authenticate(&mut self, username: GString, password: GString) {
        // Load config from client.toml in the current directory
        let config = crate::client_auth::AuthClientConfig::load(".");

        // Create channel for result
        let (tx, rx) = channel::<AuthResult>();

        // Store receiver in the struct
        self.auth_result_rx = Some(rx);

        // Spawn a thread to run the auth
        let username_str = username.to_string();
        let password_str = password.to_string();

        thread::spawn(move || {
            match crate::client_auth::authenticate(&config, &username_str, &password_str) {
                Ok(result) => {
                    if result.success {
                        let token = result.token.unwrap_or_default();
                        let game_server = result.game_server.unwrap_or_default();
                        let _ = tx.send(AuthResult::Success {
                            token,
                            game_server,
                        });
                    } else {
                        let error_msg = match result.error {
                            Some(crate::shared::AuthError::ConnectionFailed) => "ConnectionFailed",
                            Some(crate::shared::AuthError::Timeout) => "Timeout",
                            Some(crate::shared::AuthError::InvalidCredentials) => "InvalidCredentials",
                            Some(crate::shared::AuthError::ServerError) => "ServerError",
                            Some(crate::shared::AuthError::TokenExpired) => "TokenExpired",
                            None => "Unknown",
                        };
                        let _ = tx.send(AuthResult::Failure {
                            error: error_msg.to_string(),
                        });
                    }
                }
                Err(e) => {
                    let _ = tx.send(AuthResult::Failure {
                        error: format!("Auth error: {}", e),
                    });
                }
            }
        });
    }

    /// Check if the current auth token is expired.
    #[func]
    fn is_token_expired(&self) -> bool {
        if self.auth_token.is_empty() {
            return true;
        }
        let config = crate::client_auth::AuthClientConfig::load(".");
        crate::client_auth::check_token_expired(
            &self.auth_token.to_string(),
            &config.jwt_secret,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let valid_transitions = [
            ("LoadingScreen", "AuthScreen"),
            ("AuthScreen", "CharacterSelectScreen"),
            ("CharacterSelectScreen", "EmptyRoom"),
        ];

        for (from, to) in valid_transitions.iter() {
            let found = valid_transitions
                .iter()
                .any(|(f, t)| *f == *from && *t == *to);
            assert!(found, "Transition {} -> {} should be valid", from, to);
        }
    }

    #[test]
    fn test_invalid_transition_rejected() {
        let valid_transitions = [
            ("LoadingScreen", "AuthScreen"),
            ("AuthScreen", "CharacterSelectScreen"),
            ("CharacterSelectScreen", "EmptyRoom"),
        ];

        let from = "AuthScreen";
        let to = "EmptyRoom";
        let valid = valid_transitions
            .iter()
            .any(|(f, t)| *f == from && *t == to);
        assert!(!valid, "Transition {} -> {} should be invalid", from, to);
    }
}
