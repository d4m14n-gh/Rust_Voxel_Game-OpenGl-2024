use dashmap::DashMap;

use crate::{c3d3, coords::Coord3};

#[derive(Clone)]
pub struct Chunk{
    chunk_position: Coord3,
    voxels_table: [usize; Chunk::CHUNK_SIZE.pow(3)],
    voxels: Vec<usize>
}
impl Default for Chunk{
    fn default() -> Self {
        Chunk{
            chunk_position: Coord3::new(0, 0, 0),
            voxels_table: [0; Chunk::CHUNK_SIZE.pow(3)],
            voxels: Vec::new()
        }
    }
}
impl Chunk {
    const CHUNK_SIZE: usize = 8;
    pub fn is_border(local_position: Coord3) -> bool{
        (local_position.x == 0) | (local_position.x == Chunk::CHUNK_SIZE as i32-1)
        | (local_position.y == 0) | (local_position.y == Chunk::CHUNK_SIZE as i32-1)
        | (local_position.z == 0) | (local_position.z == Chunk::CHUNK_SIZE as i32-1)
    }
    pub fn is_outer(local_position: Coord3) -> bool{
        (local_position.x < 0) | (local_position.x >= Chunk::CHUNK_SIZE as i32)
        | (local_position.y < 0) | (local_position.y >= Chunk::CHUNK_SIZE as i32)
        | (local_position.z < 0) | (local_position.z >= Chunk::CHUNK_SIZE as i32)
    }
    pub fn get_index(local_position: Coord3) -> usize{
        let c3d: (usize, usize, usize) = local_position.to_usize3().unwrap();
        c3d.0*Chunk::CHUNK_SIZE.pow(2)+c3d.1*Chunk::CHUNK_SIZE+c3d.2
    }
    pub fn get_local_position_from_index(index: usize) -> Coord3{
        Coord3::new(
            (index/Chunk::CHUNK_SIZE.pow(2)) as i32,
            (( index%Chunk::CHUNK_SIZE.pow(2) )/Chunk::CHUNK_SIZE) as i32, 
            (index%Chunk::CHUNK_SIZE) as i32
        )
    }
    pub fn get_world_position(&self, local_posiotion: Coord3) -> Coord3{
        self.chunk_position*Chunk::CHUNK_SIZE as i32+local_posiotion
    }
    pub fn get_voxel(&self, local_position: Coord3) -> usize{
        self.voxels_table[Chunk::get_index(local_position)]
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
    pub fn set_chunk_position(&mut self, chunk_posiotion: Coord3){
        self.chunk_position =  chunk_posiotion;
    }
    pub fn get_voxels(&self) -> &Vec<usize>{
        &self.voxels
    }
    pub fn calculate_faces_table(&self) -> Vec<u8>{
        let mut faces_table = [0 as u8; Chunk::CHUNK_SIZE.pow(3)];
        let iterator = ChunkCoordsIterator::new();
        for local_position in iterator{
                    let index: usize = Chunk::get_index(local_position);
                    let mut mesh_type_mask: u8 = 0b00000000;
                    faces_table[index]=mesh_type_mask;
                    if self.get_voxel(local_position) == 0{
                        continue;
                    }
                    for pair in Coord3::neightbors_into_iter().zip(0..6){
                        let pos = local_position+pair.0;
                        if !Chunk::is_outer(pos) && self.get_voxel(pos) != 0{
                            mesh_type_mask |= 1<<pair.1;
                        }
                    }
                    faces_table[index]=mesh_type_mask;
        }
        faces_table.into()
    }
    //todo: rewrite
    pub fn calculate_ambient_occlusion(&self, gen: impl Fn(Coord3) -> usize) -> Vec<u32>{
        let mut ao_mask_table = [0 as u32; Chunk::CHUNK_SIZE.pow(3)];
        let iterator = ChunkCoordsIterator::new();
        for local_position in iterator{
                    let mut msk: u32 = 0;
                    for c in 0..3{
                        for yd in [1, -1]{

                            if self.get_voxel(local_position) == 0{
                                continue;
                            }
                            
                            let extra_shift = (c*2+(-yd+1)/2)*4;

                            let corners = [
                                (c3d3!(0, 1, -1), c3d3!(-1, 1, 0), c3d3!(-1, 1, -1)),
                                (c3d3!(0, 1, -1), c3d3!(1, 1, 0), c3d3!(1, 1, -1)),
                                (c3d3!(0, 1, 1), c3d3!(1, 1, 0), c3d3!(1, 1, 1)),
                                (c3d3!(0, 1, 1), c3d3!(-1, 1, 0), c3d3!(-1, 1, 1)),
                                ];
                            let mut cnt = 0;
                            for cor in corners{
                                let mut side1p = cor.0*yd; 
                                let mut side2p = cor.1*yd;
                                let mut cornerp = cor.2*yd;
                                match c {
                                    2 =>{
                                        side1p=side1p.xzy();
                                        side2p=side2p.xzy();
                                        cornerp=cornerp.xzy();
                                    } 
                                    0 => {
                                    }
                                    _ => {
                                        side1p=side1p.yxz();
                                        side2p=side2p.yxz();
                                        cornerp=cornerp.yxz();
                                    } 
                                };
                                side1p = local_position+side1p;
                                side2p = local_position+side2p;
                                cornerp = local_position+cornerp;

                                
                                let side1 = if Chunk::is_outer(side1p) {gen(self.get_world_position(side1p))!=0} else {self.get_voxel(side1p)!=0};
                                let side2 = if Chunk::is_outer(side2p) {gen(self.get_world_position(side2p))!=0} else {self.get_voxel(side2p)!=0};
                                let corner = if Chunk::is_outer(cornerp) {gen(self.get_world_position(cornerp))!=0} else {self.get_voxel(cornerp)!=0};
                                
                                let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                                if ao_type < 3{
                                    msk |= 1<<(extra_shift+cnt);
                                }
                                cnt += 1;
                            }
                        }
                            
                            
                            
                            
                            
                        // let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, -1))) != 0;
                        // let side2 = chunk_table.get_block(chunk_position, (pos).add((1, yd, 0))) != 0;
                        // let corner = chunk_table.get_block(chunk_position, (pos).add((1, yd, -1))) != 0;
                        
                        // let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        // if ao_type<case {
                        //     msk |= 1<<(extra_shift+1);
                        // }
                        
                        // let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, 1))) != 0;
                        // let side2 = chunk_table.get_block(chunk_position, (pos).add((1, yd, 0))) != 0;
                        // let corner = chunk_table.get_block(chunk_position, (pos).add((1, yd, 1))) != 0;
                        
                        // let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        // if ao_type<case {
                        //     msk |= 1<<(extra_shift+2);
                        // }

                        
                        // let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, 1))) != 0;
                        // let side2 = chunk_table.get_block(chunk_position, (pos).add((-1, yd, 0))) != 0;
                        // let corner = chunk_table.get_block(chunk_position, (pos).add((-1, yd, 1))) != 0;
                        
                        // let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        // if ao_type<case {
                        //     msk |= 1<<(extra_shift+3);
                        // }
                    }
                    ao_mask_table[Chunk::get_index(local_position)] = msk;
        }
        ao_mask_table.into()
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