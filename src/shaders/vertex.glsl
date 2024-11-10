#version 300 es
in vec3 aPos;
//in vec4 aColor;
in vec3 aTexCoord;

out vec4 vertexColor;
out vec3 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    vertexColor = vec4(1.0, 0.0, 0.0, 1.0);
    TexCoord = aTexCoord;
}
