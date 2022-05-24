extends Node

const RADIUS = 50.0;

export var rotation_speed = 0.5;
var current_angle = 0.0;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process_camera(delta):
	current_angle = fmod((current_angle + delta * rotation_speed), 180);
	
	var position = self.get_parent().get("components")[0];
	position.position = Vector3(RADIUS * cos(current_angle), 15.0, RADIUS * sin(current_angle));
