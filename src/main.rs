mod coords;
mod chunk;
mod chunk_master; 
mod chunk_loader;
mod camera;
mod world_generator;


use std::*;
use camera::Camera;
use dashmap::DashMap;
use chunk_loader::*;
use coords::*;
use chunk::*;
use gl::COLOR;
use nalgebra::Vector3;
use time::Duration;
use world_generator::{BlockType, WorldGenerator};

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
                
mod prototype;
fn main() {
    let current = thread::current();
    println!("{:?}", current.name());
    
    let mut vertices: Vec<f32> = vec![
        // -0.5, -0.5, 0.0,  // Lewy dolny
        // 0.5, -0.5, 0.0,  // Prawy dolny
        // 0.0,  0.5, 0.0,   // Górny środkowy,
        // -0.3, 0.5, 0.0,  // Lewy dolny
        // 0.3, 0.5, 0.0,  // Prawy dolny
        // 0.0,  1.5, 0.0,   // Górny środkowy
        // -0.4, 0.0, 0.0,  // Lewy dolny
        // 0.4, 0.0, 0.0,  // Prawy dolny
        // 0.0,  1.0, 0.0
        
    ];
    let wall = vec![
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, 0.5),
        Vector3::new(0.5, 0.5, 0.5),

        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, 0.5)
    ];

    
    let mut add_wall = |p: Coord3, m: f32, i:i32, c: usize|{
        for of in wall.iter(){
            let f = match i {
                1 => of.yzx(),
                2 => of.yxz(),
                _ => of.xyz()
            };
            vertices.push(p.x as f32+m*f.x);
            vertices.push(p.y as f32+m*f.y);
            vertices.push(p.z as f32+m*f.z);

            let c = WorldGenerator::get_color(c);
            vertices.push(c.x);
            vertices.push(c.y);
            vertices.push(c.z);
        }
    };

    
    let generator = WorldGenerator::default();
    let mut loader = ChunkLoader::default();
    loader.set_load_distance(5);
    
    let joins = loader.commit_world_positon();
    for j in joins.into_iter(){
        j.join().unwrap();
    }

    while true {
        if let Ok(w) = loader.get_coords_to_load().try_recv(){
            let mut chunkers = Chunk::default();
            chunkers.set_chunk_positon(w);    
            generator.generate_chunk(&mut chunkers);
            
            for index in chunkers.get_voxels(){
                let voxel_type = chunkers.get_voxel_from_index(*index);
                let pos: Coord3 = Chunk::get_local_position_from_index(*index);
                let pos = chunkers.get_world_positon(pos);
                
                //println!("{} {} {}", pos.x, pos.y, pos.z);
                //top
                
                for m in vec![-1.0, 1.0]{
                    for i in 0..3{
                        add_wall(pos, m, i, voxel_type);
                    }
                }
            }
        }
        else {
            break;
        }
    }
    prototype::draw(vertices);

    println!("program ends");
}