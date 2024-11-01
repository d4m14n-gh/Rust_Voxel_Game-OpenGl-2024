use nalgebra::Vector3;
use noise::core::value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Dirt = 2,
    Grass = 3,
    Water = 4, 
    Sand = 5,
}

impl Into<u16> for BlockType {
    fn into(self) -> u16 {
        self as u16
    }
}

impl From<u16> for BlockType {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl BlockType {
    pub fn get_color(self) -> Vector3<f32>{
        match self {
            BlockType::Dirt => Vector3::new(0.5, 0.25, 0.1), //133, 67, 18
            BlockType::Grass => Vector3::new(0.1, 0.3, 0.0),
            BlockType::Stone => Vector3::new(0.2, 0.2, 0.2),
            BlockType::Water => Vector3::new(0.05, 0.15, 0.5),
            BlockType::Sand => Vector3::new(0.7, 0.5, 0.1), //rgb(229, 192, 123)
            _ => Vector3::new(0.0, 0.0, 0.2),
        }
    }
}