// Outline, by GDQuest
// Source: https://github.com/GDQuest/godot-shaders
shader_type spatial;
render_mode cull_front, unshaded;

uniform vec4 outline_color : hint_color = vec4(0.0, 0.0, 0.0, 1.0);
uniform float outline_width = 1.0;



void vertex() {
	// Converts vertex position and normal to clip coordinates.
	vec4 clip_position = PROJECTION_MATRIX * (MODELVIEW_MATRIX * vec4(VERTEX, 1.0));
	vec3 clip_normal = mat3(PROJECTION_MATRIX) * (mat3(MODELVIEW_MATRIX) * NORMAL);

	// Calculates the width based off of view distance to keep
	// the outline width constant.
	clip_position.xy += normalize(clip_normal.xy) / VIEWPORT_SIZE * clip_position.w * outline_width * 2.0;

	POSITION = clip_position;
}



void fragment() {
	ALBEDO = outline_color.rgb;
}
