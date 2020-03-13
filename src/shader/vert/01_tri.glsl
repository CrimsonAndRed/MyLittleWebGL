attribute vec4 aPosition;
attribute vec4 aColor;
uniform mat4 uTransform;

varying lowp vec4 vColor;

void main() {
	gl_Position = uTransform * aPosition;
	vColor = aColor;
}
