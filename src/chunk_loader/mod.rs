use crate::{c3d3, coords::*, sqr};
use hashbrown::HashSet;
use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread}; 

pub struct ChunkLoader{
    world_position: Coord3,
    last_world_poition: Coord3,
    loaded_set: Arc<Mutex< HashSet<Coord3> >>,
    load_distance: i32,
    new_channel: (mpsc::Sender<Coord3>, mpsc::Receiver<Coord3>),
    old_channel: (mpsc::Sender<Coord3>, mpsc::Receiver<Coord3>)
} 

impl Default for ChunkLoader {
    fn default() -> Self {
        ChunkLoader{
            world_position: Coord3::default(),
            last_world_poition: Coord3::default(),
            loaded_set: Arc::new(Mutex::new( HashSet::new() )),
            load_distance: 10,
            new_channel: mpsc::channel(),
            old_channel: mpsc::channel()
        }
    }
}

impl ChunkLoader{
    pub const MIN_LOAD_DISTANCE: i32 = 1;
    pub const MAX_LOAD_DISTANCE: i32 = 225;

    pub fn set_load_distance(&mut self, distance: i32){
        self.load_distance = distance.clamp(
            ChunkLoader::MIN_LOAD_DISTANCE, 
            ChunkLoader::MAX_LOAD_DISTANCE
        );
    }

    pub fn set_world_positon(&mut self, new_position: Coord3){
        self.world_position = new_position;
    }
    
    pub fn commit_world_positon(&mut self){
        if self.world_position != self.last_world_poition || Arc::clone(&self.loaded_set).lock().unwrap().is_empty(){
            self.last_world_poition = self.world_position.clone();
            
            let ld = self.load_distance;
            let center = self.world_position.clone();
            let set_cloned = Arc::clone(&self.loaded_set);
            let sender = self.new_channel.0.clone();
            thread::spawn(move ||{
                for xi in -ld..ld+1{
                    for yi in -ld..ld+1{
                        for zi in -ld..ld+1{
                            let offset = c3d3!(xi, yi, zi);
                            let position = center.clone()+offset.clone();
                            if offset.magnitude2() <= sqr!(ld){
                                let mut set_gaurd = set_cloned.lock().unwrap();
                                if !set_gaurd.contains(&position){
                                    sender.send(position.clone()).unwrap();
                                    set_gaurd.insert(position);
                                }
                            }
                        }
                    }
                }
                let mut set_gaurd = set_cloned.lock().unwrap();
                for position in *set_gaurd{
                    
                }
            }); 
        }
    }
    
    pub fn get_coords_to_load(&mut self) -> &mut Receiver<Coord3>{
        &mut self.new_channel.1
    }
}