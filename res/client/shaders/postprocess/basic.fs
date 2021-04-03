#version 440 core

out vec4 o_colour;

in vec2 tex_coord;

uniform sampler2D screen_tex;

void main() 
{
	vec3 colour = texture(screen_tex, tex_coord).rgb;
	o_colour = vec4(colour, 1.0);
}
