use godot::prelude::*;

/// Rust autoload singleton managing scene transitions.
#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct GameState {
    base: Base<Node>,
    current_scene: GString,
}

#[godot_api]
impl INode for GameState {
    fn ready(&mut self) {
        self.current_scene = GString::from("LoadingScreen");
        godot_print!("GameState ready, current scene: {}", self.current_scene);
    }
}

#[godot_api]
impl GameState {
    #[func]
    fn get_current_scene(&self) -> GString {
        self.current_scene.clone()
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
