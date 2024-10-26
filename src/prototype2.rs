use gl;
use glutin::{self, event::*, event_loop::*, window::*};
use nalgebra::{self as na, Matrix, Matrix4, Vector3};

// Vertex shader z obsługą instancingu
const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aNormal;
    layout (location = 2) in vec3 aOffset;   // Pozycja instancji
    layout (location = 3) in vec3 aColor;    // Kolor instancji

    uniform mat4 projection;
    uniform mat4 view;

    out vec3 Normal;
    out vec3 FragPos;
    out vec3 Color;

    void main() {
        vec3 pos = aPos + aOffset;
        gl_Position = projection * view * vec4(pos, 1.0);
        FragPos = pos;
        Normal = aNormal;
        Color = aColor;
    }
"#;

// Fragment shader z podstawowym oświetleniem
const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec3 Normal;
    in vec3 FragPos;
    in vec3 Color;

    out vec4 FragColor;

    uniform vec3 lightPos;
    uniform vec3 viewPos;

    void main() {
        // Ambient
        float ambientStrength = 0.2;
        vec3 ambient = ambientStrength * Color;

        // Diffuse
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * Color;

        // Specular
        float specularStrength = 0.5;
        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
        vec3 specular = specularStrength * spec * vec3(1.0);

        vec3 result = ambient + diffuse + specular;
        FragColor = vec4(result, 1.0);
    }
"#;

struct VoxelRenderer {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    instance_vbo: gl::types::GLuint,
    shader_program: gl::types::GLuint,
    instance_count: i32,
}

impl VoxelRenderer {
    fn new() -> Self {
        // Dane wierzchołków dla jednego sześcianu (pozycje i normalne)
        let cube_vertices = create_cube_vertices();
        
        let mut vao = 0;
        let mut vbo = 0;
        let mut instance_vbo = 0;

        unsafe {
            // Główny VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // VBO dla geometrii sześcianu
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (cube_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                cube_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Pozycje
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normalne
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // VBO dla danych instancji
            gl::GenBuffers(1, &mut instance_vbo);
        }

        let shader_program = compile_shader_program(VERTEX_SHADER, FRAGMENT_SHADER);

        VoxelRenderer {
            vao,
            vbo,
            instance_vbo,
            shader_program,
            instance_count: 0,
        }
    }

    fn update_instances(&mut self, positions: &[(f32, f32, f32)], colors: &[(f32, f32, f32)]) {
        let mut instance_data = Vec::with_capacity(positions.len() * 6);
        for (pos, color) in positions.iter().zip(colors.iter()) {
            instance_data.extend_from_slice(&[
                pos.0, pos.1, pos.2,    // pozycja
                color.0, color.1, color.2  // kolor
            ]);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (instance_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                instance_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(self.vao);

            // Pozycja instancji
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribDivisor(2, 1);

            // Kolor instancji
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribDivisor(3, 1);
        }

        self.instance_count = positions.len() as i32;
    }

    fn render(&self, projection: &na::Matrix4<f32>, view: &na::Matrix4<f32>, light_pos: &na::Vector3<f32>, view_pos: &na::Vector3<f32>) {
        unsafe {
            gl::UseProgram(self.shader_program);

            // Ustawienie uniform zmiennych
            let proj_loc = gl::GetUniformLocation(self.shader_program, b"projection\0".as_ptr() as *const _);
            let view_loc = gl::GetUniformLocation(self.shader_program, b"view\0".as_ptr() as *const _);
            let light_pos_loc = gl::GetUniformLocation(self.shader_program, b"lightPos\0".as_ptr() as *const _);
            let view_pos_loc = gl::GetUniformLocation(self.shader_program, b"viewPos\0".as_ptr() as *const _);

            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::Uniform3f(light_pos_loc, light_pos.x, light_pos.y, light_pos.z);
            gl::Uniform3f(view_pos_loc, view_pos.x, view_pos.y, view_pos.z);

            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 36, self.instance_count);
        }
    }
}

pub fn draw(){
    // Tworzenie renderera
    let mut renderer = VoxelRenderer::new();

    // Przygotowanie danych wokseli
    let positions = vec![
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        // ... więcej pozycji
    ];

    let colors = vec![
        (1.0, 0.0, 0.0), // czerwony
        (0.0, 1.0, 0.0), // zielony
        (0.0, 0.0, 1.0), // niebieski
        // ... więcej kolorów
    ];

    // Aktualizacja danych instancji
    renderer.update_instances(&positions, &colors);

    let projection_matrix: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);

    let view_matrix: Matrix4<f32> = projection_matrix.clone();

    let light_position: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    let camera_position: Vector3<f32> = Vector3::new(0.5, -1.0, -5.0);

    // W pętli renderowania
    renderer.render(&projection_matrix, &view_matrix, &light_position, &camera_position);
} 

fn create_cube_vertices() -> Vec<f32> {
    // Zwraca wektor z pozycjami i normalnymi dla sześcianu
    vec![
        // Przód
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
        // ... podobnie dla pozostałych ścian
    ]
}

fn compile_shader_program(vertex_source: &str, fragment_source: &str) -> gl::types::GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = std::ffi::CString::new(vertex_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = std::ffi::CString::new(fragment_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}