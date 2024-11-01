mod math;
mod chunk;
mod chunk_master; 
mod chunk_loader;
mod camera;
mod block;
mod world_generator;
mod player;


use std::*;
use block::BlockType;
use camera::Camera;
use dashmap::DashMap;
use chunk_loader::*;
use chunk_master::ChunkMaster;
use math::*;
use chunk::*;
use gl::COLOR;
use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sync::{Arc, Mutex};
use time::Duration;
use world_generator::WorldGenerator;

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
    
    // let mut vertices: Vec<f32> = vec![
    //     // -0.5, -0.5, 0.0,  // Lewy dolny
    //     // 0.5, -0.5, 0.0,  // Prawy dolny
    //     // 0.0,  0.5, 0.0,   // Górny środkowy,
    //     // -0.3, 0.5, 0.0,  // Lewy dolny
    //     // 0.3, 0.5, 0.0,  // Prawy dolny
    //     // 0.0,  1.5, 0.0,   // Górny środkowy
    //     // -0.4, 0.0, 0.0,  // Lewy dolny
    //     // 0.4, 0.0, 0.0,  // Prawy dolny
    //     // 0.0,  1.0, 0.0
        
    // ];
    let vertices_arc: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let wall = vec![
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, 0.5),

        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(-0.5, 0.5, 0.5),
        Vector3::new(0.5, 0.5, 0.5)
    ];

    let perlin = Perlin::new(2);   
    let perlin2 = Perlin::new(2);   
    let add_wall = |vertices: &mut Vec<f32>, coord: Coord3, mdir: f32, axis:i32, block_type: BlockType, ao: u32|{
        let mut cnt = 0;
        for of in wall.iter(){
            let f = match axis {
                2 => of.xzy(),
                0 => of.xyz(),
                _ => of.yxz()
            };
            let vx = coord.x as f32+mdir*f.x;
            let vy = coord.y as f32+mdir*f.y;
            let vz = coord.z as f32+mdir*f.z;
            vertices.push(vx);
            vertices.push(vy);
            vertices.push(vz);
            let z = cnt%6;
            let z = match z {
                1 => 1,
                2 => 2,
                4 => 3,
                5 => 2,
                _ => 0
            };
            cnt += 1;
            let mut color = block_type.get_color();
            if block_type!=BlockType::Water && ao & 1<<((axis*2+((-mdir as i32+1)/2))*4+z) > 0{
                color = color-Vector3::new(0.1, 0.1, 0.1);
            }
            if block_type == BlockType::Water{
                let frequency = 3e-3;
                color.x = perlin.get([vx as f64 * frequency, vz as f64 * frequency]) as f32;
                color.y = perlin2.get([vx as f64 * frequency, vz as f64 * frequency]) as f32;
            }
            if block_type == BlockType::Grass{
                color.y+=(coord.z as f32/10.0).sin()/16.0;
            }
            vertices.push(color.x);
            vertices.push(color.y);
            vertices.push(color.z);
        }
    };

    
    let mut loader = ChunkLoader::default();
    loader.set_load_distance(25);
    let joins = loader.commit_world_positon();
    for handler in joins.into_iter(){
        handler.join().unwrap();
    }

    let chunk_map: DashMap<Coord3, Chunk> = DashMap::new(); 
    let generator = WorldGenerator::default();
    let chunks: Vec<Coord3> = loader.get_coords_to_load().try_iter().collect();
    let chunks_cnt = chunks.len();

    println!("start generationew");
    chunks.par_iter().for_each(|coord: &Coord3| {
            if !chunk_map.contains_key(coord){
                chunk_map.insert(*coord, Chunk::default());
            }
            let chunkers = &mut *chunk_map.get_mut(coord).unwrap();
            chunkers.set_chunk_position(*coord);    
            generator.generate_chunk(chunkers);
    });
    println!("generated");

    let cntr = Arc::new(Mutex::new(0));      
    chunks.par_iter().for_each(|coord: &Coord3| {
            let chunkers = & *chunk_map.get(coord).unwrap();
            if chunkers.is_empty(){
                return;
            }
            let faces_table = chunkers.calculate_faces_table(&chunk_map);
            let ao_table = chunkers.calculate_ambient_occlusion(&chunk_map, &faces_table);


            let vertices_mutex = Arc::clone(&vertices_arc);
            for index in chunkers.get_voxels(){
                let voxel_type = chunkers.get_voxel_from_index(*index);
                let pos: Coord3 = Chunk::get_local_position_from_index(*index);
                let pos = chunkers.get_world_position(pos);
            
                for i in 0..3{
                    for m in [1.0, -1.0]{
                        if faces_table[*index] & 1<<( i*2+(-m as i32+1)/2)==0{
                            let mut vertices = vertices_mutex.lock().unwrap();
                            add_wall(&mut vertices, pos, m, i, voxel_type, ao_table[*index]);
                        } 
                    }
                }
            }
            let cntr_mutex=cntr.clone();
            let mut cntr = cntr_mutex.lock().unwrap();
            *cntr+=1;
            println!("{}/{}", *cntr, chunks_cnt);
    });
    println!("generating ends");

    let vertices_mutex = Arc::clone(&vertices_arc);
    let vertices  = vertices_mutex.lock().unwrap();
    prototype::draw(vertices.clone());
    println!("program ends");
}