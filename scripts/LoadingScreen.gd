extends CanvasLayer

func _ready():
	_setup_input_map()
	# Simulate loading then transition to auth screen
	await get_tree().create_timer(1.0).timeout
	GameState.transition_to_auth()

func _setup_input_map():
	# move_forward
	if not InputMap.has_action("move_forward"):
		InputMap.add_action("move_forward")
		var ev = InputEventKey.new()
		ev.keycode = KEY_W
		InputMap.action_add_event("move_forward", ev)

	# move_back
	if not InputMap.has_action("move_back"):
		InputMap.add_action("move_back")
		var ev = InputEventKey.new()
		ev.keycode = KEY_S
		InputMap.action_add_event("move_back", ev)

	# move_left
	if not InputMap.has_action("move_left"):
		InputMap.add_action("move_left")
		var ev = InputEventKey.new()
		ev.keycode = KEY_A
		InputMap.action_add_event("move_left", ev)

	# move_right
	if not InputMap.has_action("move_right"):
		InputMap.add_action("move_right")
		var ev = InputEventKey.new()
		ev.keycode = KEY_D
		InputMap.action_add_event("move_right", ev)

	# camera_zoom
	if not InputMap.has_action("camera_zoom"):
		InputMap.add_action("camera_zoom")
		var ev_up = InputEventMouseButton.new()
		ev_up.button_index = MOUSE_BUTTON_WHEEL_UP
		InputMap.action_add_event("camera_zoom", ev_up)
		var ev_down = InputEventMouseButton.new()
		ev_down.button_index = MOUSE_BUTTON_WHEEL_DOWN
		InputMap.action_add_event("camera_zoom", ev_down)
