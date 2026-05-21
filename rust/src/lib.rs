use godot::prelude::*;

mod game_state;

struct OsrClientExtension;

#[gdextension]
unsafe impl ExtensionLibrary for OsrClientExtension {}

pub use game_state::GameState;
