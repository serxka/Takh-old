#version 440 core

in vec3 tex_coord;
in vec3 f_norm;
in vec3 frag_pos;

out vec4 o_colour;

uniform sampler2DArray u_tex;

const vec3 light_colour = vec3(1.0, 1.0, 1.0);
const vec3 light_pos = vec3(1.2, 1.0, 2.0);

void main()
{
	float ambient_strength = 0.2;
	vec3 ambient = ambient_strength * light_colour;
	vec3 albedo = texture(u_tex, tex_coord).rgb;
	
	vec3 light_dir = normalize(-light_pos/*  - frag_pos */);
	vec3 diffuse = max(dot(f_norm, light_dir), 0.0) * light_colour;
	
	vec3 result = (ambient + diffuse) * albedo;
	o_colour = vec4(result, 1.0);
}
