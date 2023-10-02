extends CharacterBody3D

@export var speed = 5.0
@export var sprint_speed = 15.0

# Get the gravity from the project settings to be synced with RigidBody nodes.

#const JUMP_VELOCITY = 4.5
#var gravity = ProjectSettings.get_setting("physics/3d/default_gravity")
const JUMP_VELOCITY = 10.0
var gravity = 30.0

func _physics_process(delta):
	# Add the gravity.
	if not is_on_floor():
		velocity.y -= gravity * delta

	# Handle Jump.
	if Input.is_action_just_pressed("roamer_jump") and is_on_floor():
		velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	var input_dir = Input.get_vector("roamer_left", "roamer_right", "roamer_up", "roamer_down")
	var direction = (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	var current_speed = speed
	if Input.is_action_pressed("roamer_sprint"):
		current_speed = sprint_speed
	if direction:
		velocity.x = direction.x * current_speed
		velocity.z = direction.z * current_speed
	else:
		velocity.x = move_toward(velocity.x, 0, current_speed)
		velocity.z = move_toward(velocity.z, 0, current_speed)

	move_and_slide()
