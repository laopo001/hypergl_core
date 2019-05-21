in vec4 position;
//uniform mat4 matrix;
void main(void){  
    gl_Position = position;
    gl_PointSize = 10.0;
}  