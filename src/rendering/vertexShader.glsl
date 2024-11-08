version 330 core
            layout (location = 0) in ivec3 aPos;

            uniform mat4 view;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * view * vec4(0.0f, 0.0f, 0.0f, 1.0f); 
                gl_Position = projection * view * vec4(vec3(aPos), 1.0);
                float distance = sqrt( aPos. );
                gl_PointSize = 100.0f;
            }