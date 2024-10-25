use std::{fmt::Display, ops::{Add, Sub}};

#[macro_export]
macro_rules! c3d3 {
    ($x:expr, $y:expr, $z:expr) => {
        Coord3{x: $x, y: $y, z: $z}
    };
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
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
impl Sub for Coord3{
    fn sub(self, rhs: Self) -> Self::Output {
        c3d3!(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z) 
    }
    type Output = Coord3;
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
        self.x.pow(2)+self.y.pow(2)+self.z.pow(2)
    }
    pub fn distance2(&self, other: Coord3) -> i32{
        (*self-other).magnitude2()
    }
    pub fn div_euclid(&self, value: i32) -> Coord3{
        c3d3!(self.x.div_euclid(value), self.y.div_euclid(value), self.z.div_euclid(value))
    }
    pub fn mod_euclid(&self, value: i32) -> Coord3{
        c3d3!(self.x.rem_euclid(value), self.y.rem_euclid(value), self.z.rem_euclid(value))
    }

    pub fn to_usize3(&self) -> Result<(usize, usize, usize), String>{
        if self.x<0 || self.y<0 || self.z<0{
            return Err("Wrong value".to_string());
        }
        return Ok((self.x as usize, self.y as usize, self.z as usize));
    }
}