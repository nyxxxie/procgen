#version 330 core

in VERTEX_OUT {
    vec3 color;
} IN;

out vec4 frag_color;

void main() {
    frag_color = vec4(IN.color, 1.0f);
}
