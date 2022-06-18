#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 vertexColor;
uniform float offSet = 0.0;

out vec3 ourColor;

void main() {
    gl_Position = vec4(aPos.x + offSet,aPos.y,aPos.z,1.0f);
    ourColor = vertexColor;
}
