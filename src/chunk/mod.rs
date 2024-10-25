use dashmap::DashMap;

use crate::coords::Coord3;

pub const CHUNK_SIZE: usize = 8;

#[derive(Clone)]
pub struct Chunk{
    voxels_table: [usize; CHUNK_SIZE.pow(3)],
    voxels: Vec<usize>
}
impl Default for Chunk{
    fn default() -> Self {
        Chunk{
            voxels_table: [0; CHUNK_SIZE.pow(3)],
            voxels: Vec::new()
        }
    }
}
impl Chunk {
    fn get_index(coord: Coord3) -> usize{
        let c3d: (usize, usize, usize) = coord.to_usize3().unwrap();
        c3d.0*CHUNK_SIZE.pow(2)+c3d.1*CHUNK_SIZE+c3d.2
    }
    pub fn get_voxel(&self, local_coord: Coord3) -> usize{
        self.voxels_table[Chunk::get_index(local_coord)]
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
}