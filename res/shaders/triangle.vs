#version 440 core

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec2 v_tex_coord;

uniform sampler2DArray u_tex;
uniform mat4 u_camera;
uniform mat4 u_model;
uniform mat4 u_project;

out vec3 tex_coord;

void main()
{
	gl_Position = u_project * u_camera * u_model * vec4(v_pos, 1.0);
	tex_coord = vec3(v_tex_coord, 1.0);
}
