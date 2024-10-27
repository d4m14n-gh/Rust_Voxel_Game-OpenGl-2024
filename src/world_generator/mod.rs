use std::usize;

use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};

use crate::{coords::Coord3, Chunk, ChunkCoordsIterator};

pub enum BlockType {
    Air = 0,
    Stone = 1,
    Dirt = 2,
    Grass = 3,
    Water = 4,
    Sand = 5,
}

impl BlockType {
    pub fn from_usize(value: usize) -> BlockType{
        match value {
            1 => BlockType::Stone,
            2 => BlockType::Dirt,
            3 => BlockType::Grass,
            4 => BlockType::Water,
            5 => BlockType::Sand,
            _ => BlockType::Air
        }
    }
}

pub struct WorldGenerator{
    seed: u32,
    perlin: Perlin
}
impl Default for WorldGenerator{
    fn default() -> Self {
        WorldGenerator{
            seed: 2137,
            perlin: Perlin::new(2137)
        }
    }
}
impl WorldGenerator {
    const STONE_LAYER: i32 = 5;
    const WATER_LEVEL: i32 = 4;
    pub fn get_terrein_height(&self, world_position: Coord3) -> i32{
        let frequency: f64 = 0.015;
        let frequency2: f64 = 0.15;
        let frequency3: f64 = 0.5;
        let frequency4: f64 = 0.003;
        let x = world_position.x as f64;
        let z = world_position.z as f64;

        let noise_value = self.perlin.get([x * frequency, z * frequency]);
        let noise_value2 = self.perlin.get([x * frequency2, z * frequency2]);
        let noise_value3 = self.perlin.get([x * frequency3, z * frequency3]);
        let noise_value4 = self.perlin.get([x * frequency4, z * frequency4]);
        //let elevation =  (25.*noise_value+3.5*noise_value2).round() as i32;
        //let elevation =  (10.*noise_value+0.5*noise_value2+0.15*noise_value3).round() as i32;
        let elevation =  (15.*noise_value+2.5*noise_value2+0.5*noise_value3).round() as i32+10;
        let elevation =  (10.*noise_value+1.25*noise_value2+0.25*noise_value3).round().max((25.*noise_value+2.5*noise_value2+0.5*noise_value3).round()) as i32;
        //let elevation =  (140.*noise_value4+(10.*noise_value+1.25*noise_value2+0.25*noise_value3).round().max((25.*noise_value+2.5*noise_value2+0.5*noise_value3)).round()) as i32;
        elevation
        //(7.0*(world_position.x as f32/10.0+world_position.z as f32/20.0).sin()+2.0) as i32
    }
    pub fn get_voxel_type(&self, world_position: Coord3) -> BlockType{
        let wy = world_position.y;
        let th = self.get_terrein_height(world_position);

        if world_position.distance2(Coord3::default()) > 50*50{
            return BlockType::Air;
        }     
        else if wy == th && wy > WorldGenerator::WATER_LEVEL+1{
            return BlockType::Grass
        }
        else if wy==th || (wy==th-1 && th<WorldGenerator::WATER_LEVEL) {
            return BlockType::Sand
        }
        else if wy <= th-WorldGenerator::STONE_LAYER{
            return BlockType::Stone
        }
        else if wy < th{
            return BlockType::Dirt
        }
        else if wy <= WorldGenerator::WATER_LEVEL{
            return BlockType::Water
        }
        BlockType::Air
    }
    pub fn generate_chunk(&self, chunk: &mut Chunk){
        for local_position in ChunkCoordsIterator::new(){
            let voxel_type = self.get_voxel_type(chunk.get_world_position(local_position)) as usize;
            if voxel_type != 0{
                chunk.set_voxel(local_position, voxel_type);
            }
        }
    }
    pub fn get_color(block_type: usize) -> Vector3<f32>{
        match BlockType::from_usize(block_type) {
            BlockType::Dirt => Vector3::new(0.5, 0.25, 0.1), //133, 67, 18
            BlockType::Grass => Vector3::new(0.1, 0.3, 0.0),
            BlockType::Stone => Vector3::new(0.2, 0.2, 0.2),
            BlockType::Water => Vector3::new(0.1, 0.2, 0.5),
            BlockType::Sand => Vector3::new(0.7, 0.5, 0.1), //rgb(229, 192, 123)
            _ => Vector3::new(0.0, 0.0, 0.2),
        }
    }
}