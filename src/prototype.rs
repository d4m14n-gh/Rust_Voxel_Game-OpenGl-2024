use std::ffi::CString;
use std::time::Instant;

use gl;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::{LogicalSize, PhysicalPosition};
use nalgebra::Point3;

use crate::camera::Camera;
use crate::player::Player;
use crate::Vec3;

// Vertex shader w GLSL
const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 position;
    layout(location = 1) in vec3 aColor;
    
    uniform float time;
    uniform mat4 view;
    uniform mat4 projection;

    out vec4 color;
    void main() {
        float yo = 0.0f;
        if(aColor.z > 0.35f){
            color = vec4(0.05f, 0.15f, aColor.b, 0.95f);
            vec3 direction = vec3(aColor.r, 0.0f, sqrt(1-aColor.r*aColor.r));
            float sins = sin((position.z*direction.z+position.x*direction.x)*2-time*150.0f)/16.0f;
            float sins2 = sin(sqrt(position.z*position.z+position.x*position.x)*2-time*150.0f)/16.0f;
            float sinsum = sins;//(sins+sins2)/2.0f;
            yo=((sinsum*16.0f)-1)/5.0f;
            color.b += sinsum/7.0f;
            color.g += sinsum/7.0f;
        }
        else
            color = vec4(aColor, 1.0f);
        gl_Position = projection * view * vec4(position.x, position.y+yo, position.z, 1.0);
    }
"#;

// Fragment shader w GLSL
const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec4 color;
    out vec4 FragColor;
    void main() {
        FragColor = color; // Pomarańczowy kolor
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
                let mut player = Player::new();
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
                    player.go(w, s, a, d, interval as f32*1e-6);
                    let d = delta as f32/1e6/21.0;
                    //let r = 120.0;
                    //let camera_position = Vec3::new(r*d.sin(), 30.25, r*d.cos());
                    camera.set_camera_position(player.get_position());
                    ///println!("{}", camera.get_camera_position());
                    camera.set_look_at(player.get_rotation().to_direction(Vec3::FORWARD)+camera.get_camera_position());

                        //gl::Enable(gl::BLEND);
                        //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                        //gl::DepthMask(gl::FALSE);
                        gl::Uniform1f(time_location, d);
                        let ratio = gl_window.window().inner_size().width as f32/gl_window.window().inner_size().height as f32;
                        gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, camera.get_projection_matrix(ratio).as_ptr());
                        gl::UniformMatrix4fv(view_location, 1, gl::FALSE, camera.get_view_matrix().as_ptr());
                        gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len()/6) as i32);
                    
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
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Aktualizacja viewportu po zmianie rozmiaru okna
                //gl::viewport(0, 0, size.width as i32, size.height as i32);
                unsafe {
                    gl::Viewport(0 as i32, 0 as i32, size.width as i32, size.height as i32);
                }
                println!("Nowy rozmiar okna: {:?}", size);
            },
            Event::WindowEvent { event: WindowEvent::CursorMoved { position , .. }, .. } => {
                if blocked{
                    let window = gl_window.window();
                    let size = window.inner_size();
                    let center_x = size.width as f64 / 2.0;
                    let center_y = size.height as f64 / 2.0;


                    
                    let delta_x = position.x - center_x;
                    let delta_y = position.y - center_y;
                    
                    let mouse_sensivity= 0.005;
                    player.rotate(delta_x as f32, delta_y as f32, mouse_sensivity);
                
                    window.set_cursor_position(glutin::dpi::PhysicalPosition::new(center_x, center_y))
                    .expect("Nie można ustawić pozycji kursora");
                }
            },
            Event::WindowEvent {
                event: WindowEvent::Focused(focused),
                ..
            } => {
                // Ustaw flagę w zależności od stanu fokusu
                if !focused{
                    blocked=false;
                    gl_window.window().set_cursor_visible(!blocked);
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