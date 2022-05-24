extends Spatial

var collision_exception = [];
export var min_distance = 0.5;
export var max_distance = 4.0;
export var angle_v_adjust = 0.0;
export var autoturn_ray_aperture = 25;
export var autoturn_speed = 50;
var max_height = 2.0;
var min_height = 1.0;

func _ready():
	# Find collision exceptions for ray
	var node = self.get_parent();
	while node:
		if node is RigidBody:
			collision_exception.append(node.get_rid())
			break
		else:
			node = node.get_parent()

	# This detaches the camera transform from the parent spatial node
	self.get_parent().set_as_toplevel(true)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process_camera(delta):
	var node = self.get_parent();

	var target = node.get_parent().get_global_transform().origin;
	var pos = node.get_global_transform().origin;
	var distance = pos - target;

	# Regular distance follow

	# Check ranges
	if distance.length() < min_distance:
		distance = distance.normalized() * min_distance;
	elif distance.length() > max_distance:
		distance = distance.normalized() * max_distance;

	# Check upper and lower height
	if distance.y > max_height:
		distance.y = max_height
	if distance.y < min_height:
		distance.y = min_height

	# Check autoturn
	var ds = PhysicsServer.space_get_direct_state(get_world().get_space())

	var col_left = ds.intersect_ray(target, target + Basis(Vector3.UP, deg2rad(autoturn_ray_aperture)).xform(distance), collision_exception)
	var col = ds.intersect_ray(target, target + distance, collision_exception)
	var col_right = ds.intersect_ray(target, target + Basis(Vector3.UP, deg2rad(-autoturn_ray_aperture)).xform(distance), collision_exception)

	if !col.empty():
		# If main ray was occluded, get camera closer, this is the worst case scenario
		distance = col.position - target
	elif !col_left.empty() and col_right.empty():
		# If only left ray is occluded, turn the camera around to the right
		distance = Basis(Vector3.UP, deg2rad(-delta * autoturn_speed)).xform(distance)
	elif col_left.empty() and !col_right.empty():
		# If only right ray is occluded, turn the camera around to the left
		distance = Basis(Vector3.UP, deg2rad(delta * autoturn_speed)).xform(distance)
	else:
		# Do nothing otherwise, left and right are occluded but center is not, so do not autoturn
		pass

	# Apply lookat
	if distance == Vector3.ZERO:
		distance = (pos - target).normalized() * 0.0001;

	var position = node.get("components")[0];
	position.position = target + distance;

	var yaw_pitch = node.get("components")[4];
	yaw_pitch.yaw_degrees = fmod(yaw_pitch.yaw_degrees + angle_v_adjust, 720);

	var look_at = node.get("components")[5];
	look_at.target = target;
