use std::ffi::CString;
use std::time::Instant;

use gl;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use nalgebra::Point3;

use crate::camera::Camera;

// Vertex shader w GLSL
const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 position;
    layout(location = 1) in vec3 aColor;
    
    uniform float time;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 color;
    void main() {
        gl_Position = projection * view * vec4(position.x, position.y, position.z, 1.0);
        color = aColor;
    }
"#;

// Fragment shader w GLSL
const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec3 color;
    out vec4 FragColor;
    void main() {
        FragColor = vec4(color.x, color.y, color.z, 0.2); // Pomarańczowy kolor
    }
"#;

pub fn draw(mut vertices: Vec<f32>) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("OpenGL game")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    let gl_window = ContextBuilder::new()
        .with_vsync(false)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let gl_window = unsafe { gl_window.make_current().unwrap() };

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    // Tworzenie VBO i VAO
    let mut vbo = 0;
    let mut vao = 0;
    
    unsafe {
        // Generowanie VAO
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generowanie VBO
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Ustawianie atrybutów wierzchołków
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
    }

    // Kompilacja shaderów
    let shader_program = compile_shader_program(VERTEX_SHADER, FRAGMENT_SHADER);
    let start_time: Instant = Instant::now();
    let mut frame_cnt = 0;
    let mut delta = start_time.elapsed().as_millis();

    let mut time_location = 0;
    unsafe {
        time_location = gl::GetUniformLocation(shader_program, CString::new("time").unwrap().as_ptr());
    };
    let mut projection_location = 0;
    unsafe {
        projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
    };
    let mut view_location = 0;
    let mut camera = Camera::default();
    unsafe {
        view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
    };  
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::UseProgram(shader_program);
                    gl::BindVertexArray(vao);
                    

                    let interval = start_time.elapsed().as_micros()-delta;
                    frame_cnt+=1;
                    if frame_cnt%100 == 0 {
                        println!("{interval} {}", 1000000/interval);
                    }
                    delta = start_time.elapsed().as_micros();
                    let d = delta as f32/1e6/15.0;
                    let r = 120.0;
                    let camera_position = Point3::new(r*d.sin(), 30.25, r*d.cos());
                    camera.set_camera_position(camera_position);


                        gl::Uniform1f(time_location, d);
                        gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, camera.get_projection_matrix().as_ptr());
                        gl::UniformMatrix4fv(view_location, 1, gl::FALSE, camera.get_view_matrix().as_ptr());
                        gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len()/6) as i32);
                    
                }

                gl_window.swap_buffers().unwrap();
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode, .. }, .. }, .. } => {
                if let Some(keycode) = virtual_keycode {
                    let mut camera_position = camera.get_camera_position();
                    let movement_speed = 5e-1;
                    match (keycode, state) {
                        (VirtualKeyCode::W, ElementState::Pressed) => {
                            camera_position.z -= movement_speed; // Ruch do przodu
                        }
                        (VirtualKeyCode::S, ElementState::Pressed) => {
                            camera_position.z += movement_speed; // Ruch do tyłu
                        }
                        (VirtualKeyCode::A, ElementState::Pressed) => {
                            camera_position.x -= movement_speed; // Ruch w lewo
                        }
                        (VirtualKeyCode::D, ElementState::Pressed) => {
                            camera_position.x += movement_speed; // Ruch w prawo
                        }
                        _ => {}
                    }
                    camera.set_camera_position(camera_position);
                }
            }
            _ => (),
        }
    });
}

fn compile_shader_program(vertex_shader_source: &str, fragment_shader_source: &str) -> gl::types::GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = std::ffi::CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = std::ffi::CString::new(fragment_shader_source.as_bytes()).unwrap();
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