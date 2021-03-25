#version 440 core

in vec3 tex_coord;

out vec4 o_colour;

uniform sampler2DArray u_tex;

void main()
{
	vec3 colour = texture(u_tex, tex_coord).rgb;
	o_colour = vec4(colour, 1.0f);
}
