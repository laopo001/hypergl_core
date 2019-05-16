in vec4 position;
uniform mat4 matrix;
void main(void){  
    gl_Position = matrix * position;
    gl_PointSize = 10.0;
}  