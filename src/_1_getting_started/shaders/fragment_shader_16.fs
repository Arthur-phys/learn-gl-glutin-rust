#version 330 core

in vec3 ourColor;
in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D ourTexture;
uniform sampler2D secondTexture;

void main() {
    FragColor = mix(texture(ourTexture, texCoord*2.0), texture(secondTexture, texCoord*2.0), 0.2);
}