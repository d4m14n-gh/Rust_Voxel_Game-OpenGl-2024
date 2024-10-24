mod coords;
mod chunk_loader;

use std::*;
use chunk_loader::ChunkLoader;
use io::Write;
use coords::*;
use time::Duration;


fn main() {
    let current = thread::current();
    println!("{:?}", current.name());
    
    let mut loader: ChunkLoader = ChunkLoader::default();
    loader.set_load_distance(5);
    loader.set_world_positon(c3d3!(1, 2, 3));
    loader.commit_world_positon();
    while true {
        for value in loader.get_coords_to_load().try_iter(){
            println!("{}", value);
        }

        print!("Wprowadź coś: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.as_str() {
            "q" => break, 
            _ => println!("zzz...")
        }

        //thread::sleep(Duration::from_millis(10));
    } 
    println!("program ends");
}