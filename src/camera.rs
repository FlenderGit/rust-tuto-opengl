use glm::{look_at, Mat4, Vec3};

use crate::input_handler::{InputHandler, Key};




pub struct Camera {
    position: Vec3<>,
    front: Vec3<>,
    up: Vec3<>,
    right: Vec3<>,

    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
    zoom: f32,
}

const SPEED: f32 = 0.05;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

impl Camera {
    pub fn new(position: Vec3<>) -> Self {
        Self {
            position,
            yaw: -90.0,
            pitch: 0.0,
            speed: SPEED,
            sensitivity: SENSITIVITY,
            zoom: ZOOM,

            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn view_matrix(&self) -> Mat4<> {
        look_at(
            &self.position,
            &(self.position + self.front),
            &self.up,
        )
    }

    pub fn process_mouse(&mut self, handler: &InputHandler) {

        let dx = handler.dx;
        let dy = handler.dy;

        self.yaw += dx * self.sensitivity;
        self.pitch += dy * self.sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.front.y = self.pitch.to_radians().sin();
        self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front = self.front.normalize();

        self.right = self.front.cross(&self.up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }

    pub fn process_keys(&mut self, input: &InputHandler) {
        if input.is_key_down(Key::Forward) {
            self.position += self.front * self.speed;
        }
        if input.is_key_down(Key::Backward) {
            self.position -= self.front * self.speed;
        }
        if input.is_key_down(Key::Left) {
            self.position -= self.right * self.speed;
        }
        if input.is_key_down(Key::Right) {
            self.position += self.right * self.speed;
        }
    }


}