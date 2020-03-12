precision mediump float;
uniform vec4 uColor;
uniform float uOpacity;

void main() {
	gl_FragColor = vec4(uColor.rgb, uColor.a * uOpacity);
}

