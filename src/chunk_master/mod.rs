use dashmap::DashMap;
use crate::{Chunk, Coord3};

const CHUNK_SIZE: usize = 8;

pub struct ChunkMaster(DashMap<Coord3, Chunk>);
impl Default for ChunkMaster {
    fn default() -> Self {
        ChunkMaster(DashMap::new())
    }
}
impl ChunkMaster {
    pub fn get_voxel(position: Coord3){
        let chunk_positoin = position.div_euclid(CHUNK_SIZE as i32);
        let voxel_local_positoin = position.mod_euclid(CHUNK_SIZE as i32);
    }
}