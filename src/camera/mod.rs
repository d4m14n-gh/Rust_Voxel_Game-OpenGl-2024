use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

use crate::Vec3;

pub struct Camera{
    position: Vec3,
    look_at: Vec3,
    up_vector: Vector3<f32>
}
impl Default for  Camera{
    fn default() -> Self {
        Camera{
            position: Vec3::new(0.0, 40.0, -3.25),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            up_vector: Vector3::new(0.0, 1.0, 0.0)
        }
    }
}
impl Camera {
    pub fn new() -> Self{
        Camera{
            position: Vec3::new(0.0, 40.0, -3.25),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            up_vector: Vector3::new(0.0, 1.0, 0.0)
        }
    }
    pub fn get_camera_position(&self) -> Vec3{
        self.position
    }
    pub fn set_camera_position(&mut self, new_positon: Vec3){
        self.position = new_positon;
    }
    pub fn get_view_matrix(&self) -> Matrix4<f32>{
        let forward = (self.look_at-self.position).normalize();
        let position = Point3::new(self.position.x, self.position.y, self.position.z);
        let forward: Vector3<f32> = Vector3::new(forward.x, forward.y, forward.z);
        let right: Vector3<f32> = forward.cross(&(self.up_vector)).normalize();
        let up: Vector3<f32> = right.cross(&forward);

        let rotation = Matrix4::new(
            right.x,    up.x,    -forward.x,  0.0,
            right.y,    up.y,    -forward.y,  0.0,
            right.z,    up.z,    -forward.z,  0.0,
            0.0,        0.0,      0.0,       1.0,
        );
        let translation = Matrix4::new_translation(&-position.coords);
        rotation.transpose() * translation 
    }
    pub fn get_projection_matrix(&self, ratio: f32) -> Matrix4<f32>{
        let fovy = std::f32::consts::FRAC_PI_4; // 45 stopni w radianach
        let aspect_ratio = ratio;// 16.0 / 9.0;          // typowy stosunek szerokości do wysokości ekranu
        let near = 0.1;                         // odległość do najbliższej płaszczyzny widzenia
        let far = 1000.0;                        // odległość do najdalszej płaszczyzny widzenia
        
        let perspective = Perspective3::new(aspect_ratio, fovy, near, far);
        perspective.to_homogeneous()
    }
    pub fn set_look_at(&mut self, look_at: Vec3){
        self.look_at = look_at;
    }
    pub fn get_look_at(&mut self) -> Vec3{
        self.look_at
    }
}