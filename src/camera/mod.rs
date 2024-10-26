use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

pub struct Camera{
    position: Point3<f32>,
    look_at: Point3<f32>,
    up_vector: Vector3<f32>
}
impl Default for  Camera{
    fn default() -> Self {
        Camera{
            position: Point3::new(0.0, 40.0, -3.25),
            look_at: Point3::new(0.0, 0.0, 0.0),
            up_vector: Vector3::new(0.0, 1.0, 0.0)
        }
    }
}
impl Camera {
    pub fn get_camera_position(&self) -> Point3<f32>{
        self.position
    }
    pub fn set_camera_position(& mut self, new_positon: Point3<f32>){
        self.position = new_positon;
    }
    pub fn get_view_matrix(&self) -> Matrix4<f32>{
        let forward: Vector3<f32> = (self.look_at-self.position).normalize();
        let right: Vector3<f32> = forward.cross(&(self.up_vector)).normalize();
        let up: Vector3<f32> = right.cross(&forward);

        let rotation = Matrix4::new(
            right.x,    up.x,    -forward.x,  0.0,
            right.y,    up.y,    -forward.y,  0.0,
            right.z,    up.z,    -forward.z,  0.0,
            0.0,        0.0,      0.0,       1.0,
        );
        let translation = Matrix4::new_translation(&-self.position.coords);
        rotation.transpose() * translation 
    }
    pub fn get_projection_matrix(&self) -> Matrix4<f32>{
        let fovy = std::f32::consts::FRAC_PI_4; // 45 stopni w radianach
        let aspect_ratio = 8.0/6.0;// 16.0 / 9.0;          // typowy stosunek szerokości do wysokości ekranu
        let near = 0.1;                         // odległość do najbliższej płaszczyzny widzenia
        let far = 450.0;                        // odległość do najdalszej płaszczyzny widzenia
        
        let perspective = Perspective3::new(aspect_ratio, fovy, near, far);
        let rotation = Matrix4::new(
            1.0, 0.0, 0.0,  0.0,
            0.0, 1.0, 0.0,  0.0,
            0.0, 0.0, 1.0,  0.0,
            0.0, 0.0, 0.0,  3.2,
        );
        //rotation
        perspective.to_homogeneous()
    }
}