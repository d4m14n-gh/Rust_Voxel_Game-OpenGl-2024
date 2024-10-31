use crate::{c3d3, math::*};
use dashmap::DashSet;
use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle}
}; 

pub struct ChunkLoader{
    request_world_position: Coord3,
    world_position: Coord3,
    loaded_set: DashSet<Coord3>,
    load_distance: i32,
    new_channel: (mpsc::Sender<Coord3>, mpsc::Receiver<Coord3>),
    old_channel: (mpsc::Sender<Coord3>, mpsc::Receiver<Coord3>)
}

impl Default for ChunkLoader {
    fn default() -> Self {
        ChunkLoader{
            request_world_position: Coord3::default(),
            world_position: Coord3::default(),
            loaded_set: DashSet::new(),
            load_distance: 10,
            new_channel: mpsc::channel(),
            old_channel: mpsc::channel()
        }
    }
}

impl ChunkLoader{
    pub const MIN_LOAD_DISTANCE: i32 = 1;
    pub const MAX_LOAD_DISTANCE: i32 = 25;

    pub fn set_load_distance(&mut self, distance: i32){
        self.load_distance = distance.clamp(
            ChunkLoader::MIN_LOAD_DISTANCE, 
            ChunkLoader::MAX_LOAD_DISTANCE
        );
    }

    pub fn set_world_positon(&mut self, new_position: Coord3){
        self.request_world_position = new_position;
    }

    pub fn commit_world_positon(&mut self) -> Vec<JoinHandle<()>>{
        let mut joins: Vec<JoinHandle<()>> = Vec::new(); 
        if self.request_world_position != self.world_position || self.loaded_set.is_empty(){
            self.world_position = self.request_world_position;
            
            let ld = self.load_distance;
            let center = self.world_position;
            let new_sender = self.new_channel.0.clone();
            let old_sender = self.old_channel.0.clone();
            let loaded_set_clone = self.loaded_set.clone();
            joins.push(thread::spawn(move ||{
                let mut to_remove: Vec<Coord3> = Vec::new();
                for position in loaded_set_clone.iter(){
                    if position.distance2(center) < ld.pow(2){
                        to_remove.push(*position);
                        old_sender.send(*position).unwrap();
                    }
                }
                while !to_remove.is_empty(){
                    loaded_set_clone.remove(to_remove.last().unwrap());
                }
                for xi in -ld..ld+1{
                    for yi in -ld..ld+1{
                        for zi in -ld..ld+1{
                            let offset = c3d3!(xi, yi, zi);
                            if offset.magnitude2() <= ld.pow(2){
                                let position = center+offset;
                                if !loaded_set_clone.contains(&position){
                                    new_sender.send(position).unwrap();
                                    loaded_set_clone.insert(position);
                                }
                            }
                        }
                    }
                }
            }));
        }
        joins
    }
    
    pub fn should_be_loaded(&self, value: Coord3) -> bool{
        self.world_position.distance2(value) < self.load_distance.pow(2)
    }

    pub fn get_coords_to_load(&mut self) -> &mut Receiver<Coord3>{
        &mut self.new_channel.1
    }
    
    pub fn get_coords_to_delete(&mut self) -> &mut Receiver<Coord3>{
        &mut self.old_channel.1
    }
}