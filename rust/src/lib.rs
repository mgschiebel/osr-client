use godot::prelude::*;

pub mod game_state;
pub mod shared;
pub mod jwt;
pub mod auth_server;
pub mod client_auth;
pub mod config;
pub mod logging;

struct OsrClientExtension;

#[gdextension]
unsafe impl ExtensionLibrary for OsrClientExtension {}

pub use game_state::GameState;
