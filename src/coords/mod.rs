use std::{fmt::Display, ops::Add};

#[macro_export]
macro_rules! c3d3 {
    ($x:expr, $y:expr, $z:expr) => {
        Coord3{x: $x, y: $y, z: $z}
    };
}
#[macro_export]
macro_rules! sqr {
    ($x:expr) => {
        $x * $x
    };
}

#[derive(PartialEq, Clone, Hash, Eq)] //copy?
pub struct Coord3{
    pub x: i32,
    pub y: i32,
    pub z: i32
}
impl Default for Coord3 {
    fn default() -> Self {
        Coord3{
            x: 0,
            y: 0,
            z: 0
        }
    }
}
impl Add for Coord3 {
    type Output = Coord3;

    fn add(self, rhs: Self) -> Self::Output {
        c3d3!(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}
impl Display for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{} z:{}", self.x, self.y, self.z)
    }
}

impl Coord3 {
    pub const fn from_tuple(tp: &(i32, i32, i32)) -> Coord3 {
        Coord3{
            x: tp.0,
            y: tp.1,
            z: tp.2
        }
    }
    pub const fn new(x: i32, y: i32, z: i32) -> Coord3 {
        Coord3{x: x, y: y, z: z}
    }

    pub const fn xzy(&self) -> Coord3{
        Coord3{x: self.x, y: self.z, z: self.y}
    }
    pub const fn yxz(&self) -> Coord3{
        Coord3{x: self.y, y: self.x, z: self.z}
    }
    pub const fn yzx(&self) -> Coord3{
        Coord3{x: self.y, y: self.z, z: self.x}
    }
    pub const fn zyx(&self) -> Coord3{
        Coord3{x: self.z, y: self.y, z: self.x}
    }
    pub const fn zxy(&self) -> Coord3{
        Coord3{x: self.z, y: self.x, z: self.y}
    }

    pub const fn magnitude2(&self) -> i32{
        sqr!(self.x)+sqr!(self.y)+sqr!(self.z)
    }
}