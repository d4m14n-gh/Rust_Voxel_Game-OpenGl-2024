use dashmap::DashMap;
use crate::{chunk::Chunk, math::Coord3};

const CHUNK_SIZE: usize = 16;

pub trait ChunkMaster {
    fn get_voxel(&self, world_position: Coord3) -> usize;
}
impl ChunkMaster for DashMap<Coord3, Chunk> {
    #[inline]
    fn get_voxel(&self, world_position: Coord3) -> usize{
        let chunk_position = world_position.div_euclid(CHUNK_SIZE as i32);
        let local_position = world_position.mod_euclid(CHUNK_SIZE as i32);
        if let Some(chunkerz) = self.get(&chunk_position){
            return chunkerz.get_voxel(local_position);
        }
        0
    }
}
