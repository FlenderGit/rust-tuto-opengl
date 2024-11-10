#version 300 es

precision mediump float;

out vec4 FragColor;

in vec4 vertexColor;
in vec3 TexCoord;

uniform sampler2D textureSampler;

void main() {
    FragColor = texture(textureSampler, TexCoord.xy) * vertexColor;
}
