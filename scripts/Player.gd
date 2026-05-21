extends CharacterBody3D

@onready var nav_agent = $NavigationAgent3D
var speed = 5.0

func _ready():
	nav_agent.path_desired_distance = 0.5
	nav_agent.target_desired_distance = 0.5

func _physics_process(_delta):
	var input_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_back")

	if input_dir.length() > 0:
		var direction = (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
		var target = global_position + direction * 10.0
		nav_agent.set_target_position(target)

	if nav_agent.is_navigation_finished():
		velocity = Vector3.ZERO
	else:
		var next_path_pos = nav_agent.get_next_path_position()
		var direction = (next_path_pos - global_position).normalized()
		velocity = direction * speed

	move_and_slide()
