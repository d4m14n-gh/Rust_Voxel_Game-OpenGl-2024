use crate::{block::BlockType, world_generator::WorldGenerator, Coord3};

pub enum OctaHyperChunk<T> {
    None,
    Solid(T), 
    SubChunk(SubOHCChunk<T>)
}
pub struct SubOHCChunk<T>{
    level: i32,
    leafs: Box<[OctaHyperChunk<T>; 8]>
}

pub trait VoxelWorld {
    fn get_block_type(&self, coord: Coord3) -> BlockType;
    //fn set_block_type(&mut self, coord: Coord3, block: BlockType);
    fn generate_world(&mut self, coord: Coord3, generator: &WorldGenerator);
}

impl VoxelWorld for OctaHyperChunk<BlockType>{
    fn get_block_type(&self, coord: Coord3) -> BlockType{
        match self{
            OctaHyperChunk::Solid(block) => {
                return *block;
            },
            OctaHyperChunk::SubChunk(sub_chunk) =>{
                let new_coord = coord.div_euclid(2); 
                let index = coord.mod_euclid(2).bin_hash() as usize;
                let leaf = &sub_chunk.leafs[index];
                return leaf.get_block_type(new_coord);
            },
            OctaHyperChunk::None =>{
                return BlockType::Air;
            }
        }
    } 
    // fn set_block_type(&mut self, coord: Coord3, block: BlockType){
    //     match self{
    //         OctaHyperChunk::Solid(block) => {
    //             return *block;
    //         },
    //         OctaHyperChunk::SubChunk(sub_chunk) =>{
    //             let new_coord = coord.div_euclid(2); 
    //             let index = coord.mod_euclid(2).bin_hash() as usize;
    //             let leaf = &sub_chunk.leafs[index];
    //             return leaf.get_block_type(new_coord);
    //         },
    //         OctaHyperChunk::None =>{
    //             return BlockType::Air;
    //         }
    //     }
    // } 
    fn generate_world(&mut self, coord: Coord3, generator: &WorldGenerator){
        let blocks = generator.generate_world();
        for block in blocks{

        }
        // match self{
        //     OctaHyperChunk::Solid(block) => {
        //         return *block;
        //     },
        //     OctaHyperChunk::SubChunk(sub_chunk) =>{
        //         let new_coord = coord.div_euclid(2); 
        //         let index = coord.mod_euclid(2).bin_hash() as usize;
        //         let leaf = &sub_chunk.leafs[index];
        //         return leaf.get_block_type(new_coord, generator);
        //     },
        //     OctaHyperChunk::None =>{
        //         return BlockType::Air;
        //     }
        // }
    } 
}

impl OctaHyperChunk<BlockType> {
    pub fn new() -> Self{
        OctaHyperChunk::None
    }
    pub fn get_face(&self, coord: Coord3, world :&impl VoxelWorld) -> u8{
        let mut mesh_type_mask: u8 = 0b00000000;
        let neighbors: Vec<Coord3> = Coord3::neighbors_into_iter().collect();
        let current_block = world.get_block_type(coord);
        if current_block == BlockType::Air{
            return mesh_type_mask;
        }
        for i in 0..6{
            let pos = coord+neighbors[i];
            let block_type = world.get_block_type(pos);
            if !(block_type == BlockType::Air || (current_block != BlockType::Water && block_type == BlockType::Water)){
                mesh_type_mask |= 1<<i;
            }
        }
        mesh_type_mask
    } 
}
