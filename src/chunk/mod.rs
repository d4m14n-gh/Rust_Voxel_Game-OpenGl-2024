use dashmap::DashMap;

use crate::coords::Coord3;

#[derive(Clone)]
pub struct Chunk{
    chunk_positon: Coord3,
    voxels_table: [usize; Chunk::CHUNK_SIZE.pow(3)],
    voxels: Vec<usize>
}
impl Default for Chunk{
    fn default() -> Self {
        Chunk{
            chunk_positon: Coord3::new(0, 0, 0),
            voxels_table: [0; Chunk::CHUNK_SIZE.pow(3)],
            voxels: Vec::new()
        }
    }
}
impl Chunk {
    const CHUNK_SIZE: usize = 8;
    pub fn get_index(local_positon: Coord3) -> usize{
        let c3d: (usize, usize, usize) = local_positon.to_usize3().unwrap();
        c3d.0*Chunk::CHUNK_SIZE.pow(2)+c3d.1*Chunk::CHUNK_SIZE+c3d.2
    }
    pub fn get_local_position_from_index(index: usize) -> Coord3{
        Coord3::new(
            (index/Chunk::CHUNK_SIZE.pow(2)) as i32,
            (( index%Chunk::CHUNK_SIZE.pow(2) )/Chunk::CHUNK_SIZE) as i32, 
            (index%Chunk::CHUNK_SIZE) as i32
        )
    }
    pub fn get_world_positon(&self, local_posiotion: Coord3) -> Coord3{
        self.chunk_positon*Chunk::CHUNK_SIZE as i32+local_posiotion
    }
    pub fn get_voxel(&self, local_coord: Coord3) -> usize{
        self.voxels_table[Chunk::get_index(local_coord)]
    }
    pub fn get_voxel_from_index(&self, index: usize) -> usize{
        self.voxels_table[index]
    }
    // TODO: dodac ustawianie na 0
    pub fn set_voxel(&mut self, local_coord: Coord3, value: usize){
        if value == 0{
            panic!("Not implemented yet");
        }
        let index = Chunk::get_index(local_coord);
        if self.get_voxel(local_coord) == 0{
            self.voxels.push(index);
        }
        self.voxels_table[index] = value;
    }
    pub fn set_chunk_positon(&mut self, chunk_posiotion: Coord3){
        self.chunk_positon =  chunk_posiotion;
    }
    pub fn get_voxels(&self) -> &Vec<usize>{
        &self.voxels
    }
}

pub struct ChunkCoordsIterator{
    cnt: usize
}
impl ChunkCoordsIterator {
    pub fn new() -> ChunkCoordsIterator{
        ChunkCoordsIterator{
            cnt: 0
        }
    }
}
impl Iterator for ChunkCoordsIterator {
    type Item = Coord3;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt < Chunk::CHUNK_SIZE.pow(3){
            self.cnt += 1;
            return Some(Chunk::get_local_position_from_index(self.cnt-1))
        }
        None
    }
}