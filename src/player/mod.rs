use crate::Quat;
use crate::Vec3;

pub struct Player{
    position: Vec3,
    rotation: Quat,
}
impl Player {
    pub fn new() -> Self{
        Player{
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY
        }
    }
    pub fn go(&mut self, w: bool, s: bool, a:bool, d:bool, delta_time: f32){
        let vector_up: Vec3 = Vec3::UP;
        let mut player_velocity: Vec3 = Vec3::ZERO;
        let player_speed: f32 = 30.0;
        let direction: Vec3 = self.get_rotation().to_direction(Vec3::FORWARD).normalize();
        if w{
            player_velocity += direction;
        }
        if s{
            player_velocity -= direction;
        }
        if a{
            player_velocity -= direction.cross(vector_up).normalize();
        }
        if d{
            player_velocity += direction.cross(vector_up).normalize();
        }
        self.position += player_velocity*delta_time*player_speed; 
    }
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32, mouse_sensivity: f32){      
        let pitch = delta_y*mouse_sensivity;
        let yaw = -delta_x*mouse_sensivity;
 
        self.rotation = Quat::from_rotation(yaw, Vec3::UP)*self.rotation;
        self.rotation = self.rotation*Quat::from_rotation(pitch, Vec3::RIGHT);
        self.rotation = self.rotation.normalize();

    }
    pub fn get_rotation(&self) -> Quat{
        self.rotation
    }
    pub fn get_position(&self) -> Vec3{
        self.position
    }
    pub fn set_position(&mut self, position: Vec3){
        self.position = position;
    }
}