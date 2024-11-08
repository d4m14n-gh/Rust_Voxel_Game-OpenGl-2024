use std::usize;

use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};
use rayon::range;

use crate::{block::BlockType, math::Coord3, Chunk, ChunkCoordsIterator};

pub struct WorldGenerator{
    seed: u32,
    perlin: Perlin
}
const SEED: u32 = 2137;
impl WorldGenerator {
    const STONE_LAYER: i32 = 5;
    const WATER_LEVEL: i32 = 4;
    const RANGE: i32 = 300;
    pub fn new(seed: u32) -> WorldGenerator{
        WorldGenerator{
            seed,
            perlin: Perlin::new(seed)
        }
    }
    #[inline(always)]
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
    #[inline(always)]
    pub fn get_voxel_type(&self, world_position: Coord3) -> BlockType{
        let wy = world_position.y;
        let th = self.get_terrein_height(world_position);

        if world_position.distance2(Coord3::ZERO) > Self::RANGE.pow(2){
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
        let chunk_range = WorldGenerator::RANGE/Chunk::CHUNK_SIZE as i32+3;
        if chunk.get_chunk_position().distance2(Coord3::ZERO)>(chunk_range).pow(2){
            return;
        }
        for x in 0..Chunk::CHUNK_SIZE{
            for z in 0..Chunk::CHUNK_SIZE{  
                let mut world_position = chunk.get_world_position(Coord3::new(x as i32, 0, z as i32)); 
                world_position.y = 0;
                if world_position.distance2(Coord3::ZERO)>Self::RANGE.pow(2){
                    //continue;
                }
                for y in 0..Chunk::CHUNK_SIZE{
                    let local_position = Coord3::new(x as i32, y as i32, z as i32); 
                    let world_position = chunk.get_world_position(local_position); 
                    let h = self.get_terrein_height(world_position);
                    if world_position.y > h && world_position.y >= WorldGenerator::WATER_LEVEL{
                        break;
                    }
                    let block_type = self.get_voxel_type(world_position);
                    if block_type != BlockType::Air{
                        chunk.set_voxel(local_position, block_type);
                    }
                }
            }
        }
    }
    pub fn generate_world(&self) -> Vec<(Coord3, BlockType)>{
        let mut blocks: Vec<(Coord3, BlockType)> = Vec::new();
        for x in -WorldGenerator::RANGE..-WorldGenerator::RANGE+1{
            for z in -WorldGenerator::RANGE..-WorldGenerator::RANGE+1{
                let mut world_position = Coord3::new(x, 0, z); 
                if world_position.distance2(Coord3::ZERO)>WorldGenerator::RANGE.pow(2){
                    continue;
                }
                else {
                    for y in -WorldGenerator::RANGE..self.get_terrein_height(world_position)+1{
                        world_position.y = y;
                        let block_type = self.get_voxel_type(world_position);
                        if block_type != BlockType::Air{
                            blocks.push((world_position, block_type));
                        }
                    }
                }
            }
        }
        blocks
    }
}