use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use gl;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glutin::dpi::{LogicalSize, PhysicalPosition};
use nalgebra::Point3;

use crate::camera::Camera;
use crate::player::{self, Player};
use crate::{Coord3, Vec3};

// Vertex shader w GLSL
const VERTEX_SHADER: &str = r#"
            #version 330 core
            layout (location = 0) in ivec3 aPos;

            uniform mat4 view;
            uniform vec3 camPos;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * view * vec4(vec3(aPos), 1.0);
                gl_PointSize = 115.0;
            }
        "#;
const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main() {
        FragColor = vec4(0.0, 1.0, 0.0, 1.0);
    }
"#;

const GEOMETRY_SHADER: &str = r#"
    #version 330 core
    layout (points) in;  // Przyjmujemy punkt jako wejście (jeden wierzchołek)
    layout (triangle_strip, max_vertices = 24) out;  // Generujemy 24 wierzchołki jako wyjście (6 ścianek, 2 trójkąty na każdą)

    void main() {
        // Pobrane dane z wejściowego punktu
        vec4 vertexPosition = gl_in[0].gl_Position;

        // Współrzędne wierzchołków kostki względem punktu wejściowego
        // Przesunięcia w kierunkach X, Y, Z aby utworzyć sześcian wokół punktu
        float size = 0.5; // Rozmiar kostki

        // Wierzchołki kostki
        vec4 vertices[8];
        vertices[0] = vertexPosition + vec4(-size, -size, -size, 0.0);  // Lewy dolny tylny róg
        vertices[1] = vertexPosition + vec4( size, -size, -size, 0.0);  // Prawy dolny tylny róg
        vertices[2] = vertexPosition + vec4(-size,  size, -size, 0.0);  // Lewy górny tylny róg
        vertices[3] = vertexPosition + vec4( size,  size, -size, 0.0);  // Prawy górny tylny róg
        vertices[4] = vertexPosition + vec4(-size, -size,  size, 0.0);  // Lewy dolny przedni róg
        vertices[5] = vertexPosition + vec4( size, -size,  size, 0.0);  // Prawy dolny przedni róg
        vertices[6] = vertexPosition + vec4(-size,  size,  size, 0.0);  // Lewy górny przedni róg
        vertices[7] = vertexPosition + vec4( size,  size,  size, 0.0);  // Prawy górny przedni róg

        // Generowanie ścianek sześcianu jako dwa trójkąty na każdą ściankę
        // Ściana 1 - tylna
        gl_Position = vertices[0]; EmitVertex();
        gl_Position = vertices[2]; EmitVertex();
        gl_Position = vertices[1]; EmitVertex();
        gl_Position = vertices[3]; EmitVertex();
        EndPrimitive();

        // Ściana 2 - przednia
        gl_Position = vertices[4]; EmitVertex();
        gl_Position = vertices[5]; EmitVertex();
        gl_Position = vertices[6]; EmitVertex();
        gl_Position = vertices[7]; EmitVertex();
        EndPrimitive();

        // Ściana 3 - lewa
        gl_Position = vertices[0]; EmitVertex();
        gl_Position = vertices[4]; EmitVertex();
        gl_Position = vertices[2]; EmitVertex();
        gl_Position = vertices[6]; EmitVertex();
        EndPrimitive();

        // Ściana 4 - prawa
        gl_Position = vertices[1]; EmitVertex();
        gl_Position = vertices[5]; EmitVertex();
        gl_Position = vertices[3]; EmitVertex();
        gl_Position = vertices[7]; EmitVertex();
        EndPrimitive();

        // Ściana 5 - górna
        gl_Position = vertices[2]; EmitVertex();
        gl_Position = vertices[6]; EmitVertex();
        gl_Position = vertices[3]; EmitVertex();
        gl_Position = vertices[7]; EmitVertex();
        EndPrimitive();

        // Ściana 6 - dolna
        gl_Position = vertices[0]; EmitVertex();
        gl_Position = vertices[1]; EmitVertex();
        gl_Position = vertices[4]; EmitVertex();
        gl_Position = vertices[5]; EmitVertex();
        EndPrimitive();
    }
"#;

pub struct AppWraper{
    vbo_voxels: u32,
    vao_voxels: u32
} 

impl AppWraper {
    pub fn new() -> Self{
        AppWraper{
            vao_voxels: 0,
            vbo_voxels: 0
        }
    }

    fn create_vbo_vao(&mut self, voxels: &Vec<i32>){
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao_voxels);
            gl::BindVertexArray(self.vao_voxels);
            gl::GenBuffers(1, &mut self.vbo_voxels);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_voxels);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (voxels.len() * 4) as gl::types::GLsizeiptr,
                voxels.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribIPointer(
                0,
                3,
                gl::INT,
                3*4,
                std::ptr::null(),
            );
            //gl::EnableVertexAttribArray(1);
            // gl::VertexAttribPointer(
            //     1,
            //     1,
            //     gl::BYTE,
            //     gl::FALSE,
            //     3 * 4 + 1,
            //     (3 * 4) as *const () as *const _,
            // );
        }
    }
    pub fn run(&mut self, voxels: Vec<i32>){
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
        
        self.create_vbo_vao(&voxels);
        let shader_program = compile_shader_program(VERTEX_SHADER, FRAGMENT_SHADER, GEOMETRY_SHADER);
        
        let vao = self.vao_voxels;
        

        let mut projection_location = 0;
        unsafe { projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());};
        let mut view_location = 0;
        unsafe { view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());};
        let mut cam_pos_location = 0;
        unsafe { cam_pos_location = gl::GetUniformLocation(shader_program, CString::new("camPos").unwrap().as_ptr());};    
        let mut frame_cnt = 0;
        let mut camera = Camera::new();
        let mut player = Player::new();
        player.set_position(Vec3::new(0.0, 0.0, -3.1));

        let start_time: Instant = Instant::now();
        let mut delta = start_time.elapsed().as_millis();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }
        let mut blocked = false;
        let (mut w, mut s, mut a, mut d) = (false, false, false, false);
        
        event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::LoopDestroyed => return,
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                        *control_flow = ControlFlow::Exit
                    },
                    Event::MainEventsCleared => {
                        let interval = start_time.elapsed().as_micros()-delta;                           
                        delta = start_time.elapsed().as_micros();
                        frame_cnt+=1;
                        if frame_cnt%100 == 0 {
                            println!("{interval} {}", 1000000/interval);
                        }
                        player.go(w, s, a, d, interval as f32*1e-6);

                        camera.set_camera_position(player.get_position());    
                        camera.set_look_at(player.get_rotation().to_direction(Vec3::FORWARD)+camera.get_camera_position());
                        
                        unsafe {
                            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                            gl::BindVertexArray(vao);
                            gl::UseProgram(shader_program);
                            
                            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, camera.get_projection_matrix(8 as f32/6 as f32).as_ptr());
                            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, camera.get_view_matrix().as_ptr());
                            gl::Uniform3f(cam_pos_location, player.get_position().x, player.get_position().y, player.get_position().z);
                            gl::DrawArrays(gl::POINTS, 0, (voxels.len()/3) as i32);
                        }
                        gl_window.swap_buffers().unwrap();
                    },
                    Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode, .. }, .. }, .. } => { 
                        if let Some(keycode) = virtual_keycode {
                            match (keycode, state) {
                                (VirtualKeyCode::W, ElementState::Pressed) => {
                                    w=true;
                                }
                                (VirtualKeyCode::S, ElementState::Pressed) => {
                                    s=true;
                                }
                                (VirtualKeyCode::A, ElementState::Pressed) => {
                                    a=true;
                                }
                                (VirtualKeyCode::D, ElementState::Pressed) => {
                                    d=true;
                                }
                                (VirtualKeyCode::W, ElementState::Released) => {
                                    w=false;
                                }
                                (VirtualKeyCode::S, ElementState::Released) => {
                                    s=false;
                                }
                                (VirtualKeyCode::A, ElementState::Released) => {
                                    a=false;
                                }
                                (VirtualKeyCode::D, ElementState::Released) => {
                                    d=false;
                                }
        
                                (VirtualKeyCode::E, ElementState::Pressed) => {
                                    blocked=!blocked;
                                    let window = gl_window.window();
                                    let size = window.inner_size();
                                    let center_x = size.width as f64 / 2.0;
                                    let center_y = size.height as f64 / 2.0;
        
                                    window.set_cursor_position(glutin::dpi::PhysicalPosition::new(center_x, center_y))
                                    .expect("Nie można ustawić pozycji kursora");
                                    gl_window.window().set_cursor_visible(!blocked);
                                }
                                _ => {}
                            }
                        }
                    },
                    Event::WindowEvent { event: WindowEvent::Resized(size), ..} => {  
                    },
                    Event::WindowEvent { event: WindowEvent::CursorMoved { position , .. }, .. } => {
                        if blocked{
                            let window = gl_window.window();
                            let size = window.inner_size();
                            let center_x = size.width as f64 / 2.0;
                            let center_y = size.height as f64 / 2.0;
                            let delta_x = position.x - center_x;
                            let delta_y = position.y - center_y;
                        
                            player.rotate(delta_x as f32, delta_y as f32, 0.005);
                            window.set_cursor_position(glutin::dpi::PhysicalPosition::new(center_x, center_y))
                            .expect("Nie można ustawić pozycji kursora");
                        }
                    },
                    Event::WindowEvent { event: WindowEvent::Focused(focused), ..} => {
                        if !focused{
                            blocked=false;
                            gl_window.window().set_cursor_visible(!blocked);
                        }
                    }
                    _ => (),
                }
            }
        );
    }
    fn compile_shader_program(&mut self){
        

    }
}


fn compile_shader_program(vertex_shader_source: &str, fragment_shader_source: &str, geometry_shader_source: &str) -> gl::types::GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = std::ffi::CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0u8; 1024];
            gl::GetShaderInfoLog(vertex_shader, log.len() as i32, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            println!("Vertex Shader compilation failed: {:?}", String::from_utf8_lossy(&log));
        }


        let geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
        let c_str_vert = std::ffi::CString::new(geometry_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(geometry_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(geometry_shader);

        let mut success = 0;
        gl::GetShaderiv(geometry_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0u8; 1024];
            gl::GetShaderInfoLog(geometry_shader, log.len() as i32, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            println!("Geometry Shader compilation failed: {:?}", String::from_utf8_lossy(&log));
        }



        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = std::ffi::CString::new(fragment_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0u8; 1024];
            gl::GetShaderInfoLog(fragment_shader, log.len() as i32, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            println!("Fragment Shader compilation failed: {:?}", String::from_utf8_lossy(&log));
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, geometry_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0u8; 1024];
            gl::GetProgramInfoLog(program, log.len() as i32, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            println!("Program linking failed: {:?}", String::from_utf8_lossy(&log));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}