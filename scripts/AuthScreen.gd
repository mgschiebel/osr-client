extends Control

func _on_enter_pressed():
	GameState.transition_to_character_select()
