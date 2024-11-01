use dashmap::DashMap;
use crate::{block::BlockType, chunk::Chunk, math::Coord3};

pub trait ChunkMaster {
    fn get_voxel(&self, world_position: Coord3) -> BlockType;
}
impl ChunkMaster for DashMap<Coord3, Chunk> {
    #[inline]
    fn get_voxel(&self, world_position: Coord3) -> BlockType{
        let chunk_position = world_position.div_euclid(Chunk::CHUNK_SIZE as i32);
        let local_position = world_position.mod_euclid(Chunk::CHUNK_SIZE as i32);
        if let Some(chunkerz) = self.get(&chunk_position){
            return chunkerz.get_voxel(local_position);
        }
        BlockType::Air
    }
}
