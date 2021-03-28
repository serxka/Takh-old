#version 440 core

in vec3 tex_coord;

out vec4 o_colour;

uniform sampler2DArray u_tex;

void main()
{
	o_colour = vec4(texture(u_tex, tex_coord).rgb, 1.0);
}
