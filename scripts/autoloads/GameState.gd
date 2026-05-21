extends Node

var current_scene = "LoadingScreen"

func get_current_scene():
	return current_scene

func transition_to(scene_name: String):
	var valid_transitions = {
		"LoadingScreen": ["AuthScreen"],
		"AuthScreen": ["CharacterSelectScreen"],
		"CharacterSelectScreen": ["EmptyRoom"]
	}

	if valid_transitions.get(current_scene, []).has(scene_name):
		current_scene = scene_name
		var path = "res://scenes/"
		match scene_name:
			"AuthScreen":
				path += "auth/AuthScreen.tscn"
			"CharacterSelectScreen":
				path += "character_select/CharacterSelectScreen.tscn"
			"EmptyRoom":
				path += "world/EmptyRoom.tscn"
		get_tree().change_scene_to_file(path)
	else:
		push_error("Invalid transition: " + current_scene + " -> " + scene_name)

func transition_to_auth():
	transition_to("AuthScreen")

func transition_to_character_select():
	transition_to("CharacterSelectScreen")

func transition_to_world():
	transition_to("EmptyRoom")
