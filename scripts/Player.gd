extends CharacterBody3D

var speed = 5.0

func _physics_process(_delta):
	var input_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_back")
	if input_dir.length() > 0:
		var direction = (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
		velocity = direction * speed
	else:
		velocity = Vector3.ZERO
	move_and_slide()
