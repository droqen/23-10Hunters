extends Node3D

const DRAG_TURN_SPEED = 0.1
const HOVER_TURN_SPEED = -0.04

const MIN_PITCH = -0.5
const MAX_PITCH = 0.5

var last_mouse_position : Vector2
var yaw : float = 0.0
var pitch : float = 0.0 # loop

func _ready():
	last_mouse_position = get_viewport().get_mouse_position()

func _process(delta):
	var mouse_position = get_viewport().get_mouse_position()
	var mv = mouse_position - last_mouse_position
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		yaw += delta * mv.x * DRAG_TURN_SPEED
		pitch += delta * mv.y * DRAG_TURN_SPEED
	else:
		yaw += delta * mv.x * HOVER_TURN_SPEED
		pitch += delta * mv.y * HOVER_TURN_SPEED
	pitch = clamp(pitch, MIN_PITCH, MAX_PITCH)
	rotation.x = pitch# = Vector3(pitch, yaw, 0.0)
	get_parent().rotation.y = yaw
	last_mouse_position = mouse_position
