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
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, 0.5),

        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(-0.5, 0.5, 0.5),
        Vector3::new(0.5, 0.5, 0.5)
    ];

    
    let mut add_wall = |p: Coord3, m: f32, i:i32, c: usize, ao: u32|{
        let mut cnt = 0;
        for of in wall.iter(){
            let f = match i {
                2 => of.xzy(),
                0 => of.xyz(),
                _ => of.yxz()
            };
            vertices.push(p.x as f32+m*f.x);
            vertices.push(p.y as f32+m*f.y);
            vertices.push(p.z as f32+m*f.z);

            let z = cnt%6;
            let z = match z {
                1 => 1,
                2 => 2,
                4 => 3,
                5 => 2,
                _ => 0
            };
            let mut c = WorldGenerator::get_color(c);
            if ao & 1<<((i*2+((-m as i32+1)/2))*4+z) > 0{
                c = c-Vector3::new(0.1, 0.1, 0.1);
            }
            cnt += 1;
            vertices.push(c.x);
            vertices.push(c.y+(p.z as f32/10.0).sin()/16.0);
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

    for w in loader.get_coords_to_load().try_iter(){
            let mut chunkers = Chunk::default();
            chunkers.set_chunk_position(w);    
            generator.generate_chunk(&mut chunkers);
            
            println!("Calculating faces table!");
            let faces_table = chunkers.calculate_faces_table();
            
            let c = |world_position: Coord3| generator.get_voxel_type(world_position) as usize;
            println!("Calculating ambient occlusion!");
            let ao_table = chunkers.calculate_ambient_occlusion(c);
            
            println!("Adding walls!!");
            for index in chunkers.get_voxels(){
                let voxel_type = chunkers.get_voxel_from_index(*index);
                let pos: Coord3 = Chunk::get_local_position_from_index(*index);
                let pos = chunkers.get_world_position(pos);
                
                //println!("{} {} {}", pos.x, pos.y, pos.z);
                //top
                
                
                for i in 0..3{
                    for m in [1.0, -1.0]{
                        if faces_table[*index] & 1<<( i*2+(-m as i32+1)/2)==0{
                            add_wall(pos, m, i, voxel_type, ao_table[*index]);
                        } 
                    }
                }
            }
    }
    prototype::draw(vertices);

    println!("program ends");
}