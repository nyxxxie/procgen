#version 330 core

layout (location = 0) in vec3 vert_pos;

uniform mat4 modelview;
uniform mat4 projection;

out vec3 color;

void main() 
{
	vec4 pos = vec4(vert_pos.xyz, 1.0);
	gl_Position = projection * modelview * pos;
}
