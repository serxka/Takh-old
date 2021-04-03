#version 440 core

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_norm;
layout (location = 2) in vec3 v_tex_coord;

uniform sampler2DArray u_tex;
uniform mat4 u_camera;
uniform mat4 u_model;
uniform mat4 u_project;

out vec3 tex_coord;
out vec3 f_norm;
out vec3 frag_pos;

void main()
{
	gl_Position = u_project * u_camera * u_model * vec4(v_pos, 1.0);
	f_norm = v_norm;
	frag_pos = vec3(u_model * vec4(v_pos, 1.0));
	tex_coord = vec3(vec2(1.0) / textureSize(u_tex, 0).xy, 1.0) * v_tex_coord;
}
