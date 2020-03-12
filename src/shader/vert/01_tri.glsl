attribute vec4 position;
uniform mat4 uTransform;
void main() {
	gl_Position = uTransform * position;
}
