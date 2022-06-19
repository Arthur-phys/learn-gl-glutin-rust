#version 330 core

in vec3 ourColor;
in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D ourTexture;
uniform sampler2D secondTexture;

void main() {
    FragColor = mix(texture(ourTexture, texCoord), texture(secondTexture, texCoord), 9.0*texCoord.x*texCoord.y*(1.0-texCoord.x)*(1.0-texCoord.y));
}