#version 330 core

layout (location = 0) in vec3 vert_pos;
layout (location = 1) in vec3 vert_col;

out VERTEX_OUT {
    vec3 color;
} OUT;

void main() {
    gl_Position = vec4(vert_pos, 1.0);
    OUT.color = vert_col;
}
