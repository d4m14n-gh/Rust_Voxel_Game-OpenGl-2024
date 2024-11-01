
use crate::{c3d3, chunk_master::ChunkMaster, math::Coord3, block::BlockType};

#[derive(Clone)]
pub struct Chunk{
    chunk_position: Coord3,
    voxels_table: [BlockType; Chunk::CHUNK_SIZE.pow(3)],
    voxels: Vec<usize>
}
impl Default for Chunk{
    fn default() -> Self {
        Chunk{
            chunk_position: Coord3::new(0, 0, 0),
            voxels_table: [BlockType::Air; Chunk::CHUNK_SIZE.pow(3)],
            voxels: Vec::new()
        }
    }
}
impl Chunk {
    pub const CHUNK_SIZE: usize = 20;
    #[inline(always)]
    pub fn is_border(local_position: Coord3) -> bool{
        (local_position.x == 0) | (local_position.x == Chunk::CHUNK_SIZE as i32-1)
        | (local_position.y == 0) | (local_position.y == Chunk::CHUNK_SIZE as i32-1)
        | (local_position.z == 0) | (local_position.z == Chunk::CHUNK_SIZE as i32-1)
    }
    #[inline(always)]
    pub fn is_outer(local_position: Coord3) -> bool{
        (local_position.x < 0) | (local_position.x >= Chunk::CHUNK_SIZE as i32)
        | (local_position.y < 0) | (local_position.y >= Chunk::CHUNK_SIZE as i32)
        | (local_position.z < 0) | (local_position.z >= Chunk::CHUNK_SIZE as i32)
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool{
        self.voxels.len() == 0
    }
    #[inline(always)]
    pub fn get_index(local_position: Coord3) -> usize{
        let c3d: (usize, usize, usize) = local_position.to_usize3().unwrap();
        c3d.0*Chunk::CHUNK_SIZE.pow(2)+c3d.1*Chunk::CHUNK_SIZE+c3d.2
    }
    #[inline(always)]
    pub fn get_local_position_from_index(index: usize) -> Coord3{
        Coord3::new(
            (index/Chunk::CHUNK_SIZE.pow(2)) as i32,
            (( index%Chunk::CHUNK_SIZE.pow(2) )/Chunk::CHUNK_SIZE) as i32, 
            (index%Chunk::CHUNK_SIZE) as i32
        )
    }
    #[inline(always)]
    pub fn get_world_position(&self, local_posiotion: Coord3) -> Coord3{
        self.chunk_position*Chunk::CHUNK_SIZE as i32+local_posiotion
    }
    #[inline(always)]
    pub fn get_voxel(&self, local_position: Coord3) -> BlockType{
        self.voxels_table[Chunk::get_index(local_position)]
    }
    #[inline(always)]
    pub fn get_voxel_from_index(&self, index: usize) -> BlockType{
        self.voxels_table[index]
    }
    // TODO: dodac ustawianie na 0
    #[inline(always)]
    pub fn set_voxel(&mut self, local_coord: Coord3, value: BlockType){
        if value == BlockType::Air{
            panic!("Not implemented yet");
        }
        let index = Chunk::get_index(local_coord);
        if self.get_voxel(local_coord) == BlockType::Air{
            self.voxels.push(index);
        }
        self.voxels_table[index] = value;
    }
    #[inline(always)]
    pub fn get_chunk_position(&self) -> Coord3{
        self.chunk_position
    }
    #[inline(always)]
    pub fn set_chunk_position(&mut self, chunk_posiotion: Coord3){
        self.chunk_position =  chunk_posiotion;
    }
    #[inline(always)]
    pub fn get_voxels(&self) -> &Vec<usize>{
        &self.voxels
    }
    pub fn calculate_faces_table(&self, master: &impl ChunkMaster) -> Vec<u8>{
        let mut faces_table = [0 as u8; Chunk::CHUNK_SIZE.pow(3)];
        let neighbors: Vec<Coord3> = Coord3::neighbors_into_iter().collect();
        for indexref in self.voxels.iter(){
                    let index: usize = *indexref;
                    let local_position = Chunk::get_local_position_from_index(index);
                    let current_block = self.get_voxel(local_position);
                    let mut mesh_type_mask: u8 = 0b00000000;
                    faces_table[index]=mesh_type_mask;
                    
                    if current_block == BlockType::Air{
                        continue;
                    }
                    for i in 0..6{
                        let pos = local_position+neighbors[i];
                        let block_type = if Chunk::is_outer(pos){
                            let world_position = self.get_world_position(pos);
                            master.get_voxel(world_position)
                        }
                        else{
                            self.get_voxel(pos)
                        };
                        
                        if !(block_type == BlockType::Air || (current_block != BlockType::Water && block_type == BlockType::Water)){
                            mesh_type_mask |= 1<<i;
                        }
                    }
                    faces_table[index]=mesh_type_mask;
        }
        faces_table.into()
    }
    //todo: rewrite
    pub fn calculate_ambient_occlusion(&self, master: &impl ChunkMaster, faces_table: &Vec<u8>) -> Vec<u32>{
        let mut ao_mask_table = [0 as u32; Chunk::CHUNK_SIZE.pow(3)];
        for indexref in self.voxels.iter(){
                    let index: usize = *indexref;
                    let local_position = Chunk::get_local_position_from_index(index);
                    let mut msk: u32 = 0;
                    for axis in 0..3{
                        for direct in [1, -1]{

                            if faces_table[index] == 0{
                                continue;
                            }
                            
                            let extra_shift = (axis*2+(-direct+1)/2)*4;

                            let corners = [
                                (c3d3!(0, 1, -1), c3d3!(-1, 1, 0), c3d3!(-1, 1, -1)),
                                (c3d3!(0, 1, -1), c3d3!(1, 1, 0), c3d3!(1, 1, -1)),
                                (c3d3!(0, 1, 1), c3d3!(1, 1, 0), c3d3!(1, 1, 1)),
                                (c3d3!(0, 1, 1), c3d3!(-1, 1, 0), c3d3!(-1, 1, 1)),
                                ];
                            let mut cnt = 0;
                            for cor in corners{
                                let mut side1p = cor.0*direct; 
                                let mut side2p = cor.1*direct;
                                let mut cornerp = cor.2*direct;
                                match axis {
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

                                let side1_block = if Chunk::is_outer(side1p) {master.get_voxel(self.get_world_position(side1p))} else {self.get_voxel(side1p)};
                                let side2_block = if Chunk::is_outer(side2p) {master.get_voxel(self.get_world_position(side2p))} else {self.get_voxel(side2p)};
                                let corner_block = if Chunk::is_outer(cornerp) {master.get_voxel(self.get_world_position(cornerp))} else {self.get_voxel(cornerp)};
                                
                                let side1 = side1_block!=BlockType::Air&&side1_block!=BlockType::Water;
                                let side2 = side2_block!=BlockType::Air&&side2_block!=BlockType::Water;
                                let corner = corner_block!=BlockType::Air&&corner_block!=BlockType::Water;
                                
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