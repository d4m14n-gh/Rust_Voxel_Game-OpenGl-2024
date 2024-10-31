use std::{array::IntoIter, fmt::Display, ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}};

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
    #[inline]
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

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        c3d3!(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}
impl Sub for Coord3{
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        c3d3!(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z) 
    }
    type Output = Coord3;
}
impl Display for Coord3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{} z:{}", self.x, self.y, self.z)
    }
}
impl Mul<i32> for Coord3 {
    type Output = Coord3;
    #[inline]
    fn mul(self, scalar: i32) -> Coord3 {
        c3d3!(self.x*scalar, self.y*scalar, self.z*scalar)
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
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Coord3 {
        Coord3{x, y, z}
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

    #[inline]
    pub const fn magnitude2(&self) -> i32{
        self.x.pow(2)+self.y.pow(2)+self.z.pow(2)
    }
    #[inline]
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
            return Err("Wrong value".to_string())
        }
        Ok((self.x as usize, self.y as usize, self.z as usize))
    }
    pub fn upper(&self) -> Coord3{
        *self+c3d3!(0, 1, 0)
    }
    pub fn lower(&self) -> Coord3{
        *self+c3d3!(0, -1, 0)
    }
    #[inline]
    pub fn neighbors_into_iter() -> IntoIter<Coord3, 6>{
        const NEIGHTBORS: [Coord3; 6]  = [
            c3d3!(0, 1, 0), c3d3!(0, -1, 0), c3d3!(1, 0, 0), 
            c3d3!(-1, 0, 0), c3d3!(0, 0, 1), c3d3!(0, 0, -1)
        ];
        NEIGHTBORS.into_iter()
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Default for Vec3 {
    fn default() -> Self {
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}
impl Vec3 {
    pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const ZERO: Vec3 = Vec3::new(0.0,0.0, 0.0);
    pub const fn new(x: f32, y:f32, z:f32) -> Vec3{
        Vec3{
            x: x,
            y: y,
            z: z
        }
    }
    pub fn magnitude(&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).powf(0.5)
    }
    pub fn normalize(&self) -> Vec3{
        *self/self.magnitude()
    }
    pub fn cross(&self, other: Vec3) -> Vec3{
        Vec3::new(self.y*other.z-self.z*other.y, self.z*other.x-self.x*other.z, self.x*other.y-self.y*other.x)
    }
    pub fn to_tuple(&self) -> (f32, f32, f32){
        (self.x, self.y, self.z)
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}
impl Sub for Vec3{
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z) 
    }
    type Output = Vec3;
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self+rhs;
    }
}
impl SubAssign for Vec3{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self-rhs;
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{} z:{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    pub const ZERO: Self =  Quat::new(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self =  Quat::new(1.0, 0.0, 0.0, 0.0);
    pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Quat { w, x, y, z }
    }
    pub const fn from_vec3(w: f32, vec3: Vec3) -> Quat{
        Quat{
            w,
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
        }
    }
    fn magnitude(&self) -> f32 {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Quat::new(self.w / mag, self.x / mag, self.y / mag, self.z / mag)
    }
    fn conjugate(&self) -> Self {
        Quat {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
    fn inverse(&self) -> Option<Self> {
        let mag_squared = self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        
        if mag_squared == 0.0 {
            None
        } else {
            Some(Quat {
                w: self.w / mag_squared,
                x: -self.x / mag_squared,
                y: -self.y / mag_squared,
                z: -self.z / mag_squared
            })
        }
    }
    pub fn from_rotation(angle: f32, axis: Vec3) -> Self {
        let half_angle = angle / 2.0;
        let (x, y, z) = axis.to_tuple();
        let sin_half = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            x: x * sin_half,
            y: y * sin_half,
            z: z * sin_half
        }
    }
    fn get_rotation_axis(&self) -> Vec3{
        Vec3::new(self.x, self.y, self.z)
    } 
    pub fn to_direction(&self, vec3: Vec3) -> Vec3 {
        //qvq'
        (*self*Quat::from_vec3(0.0, vec3)*self.conjugate()).get_rotation_axis()
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Quat {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w
        }
    }
}