mod coords;
mod chunk;
//mod chunk_master; 
mod chunk_loader;


use std::*;
use dashmap::DashMap;
use chunk_loader::*;
use coords::*;
use chunk::*;
use log::{info, warn};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
// for value in loader.get_coords_to_load().try_iter(){
//     println!("{}", value);
//     master_map.insert(value, Chunk::default());
    
//     let mut chunkers = master_map.get_mut(&value).unwrap();
//     for xi in 0..CHUNK_SIZE/2{
//         for yi in 0..CHUNK_SIZE{
//             for zi in 0..CHUNK_SIZE{
//                 let local = c3d3!(xi as i32, yi as i32, zi as i32);
//                 chunkers.set_voxel(local, 1);
//             }    
//         }        
//     }
// }

fn main() {
    let current = thread::current();
    println!("{:?}", current.name());
    
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| 
        match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } 
        if window_id == window.id() => 
            match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                _ => {}
            },
        _ => {}
    }).unwrap();

    println!("program ends");
}