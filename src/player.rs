use macroquad::{
    input::{is_key_down, KeyCode},
    time::get_frame_time,
};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, angle: f32) -> Self {
        Self { x, y, speed, angle }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time() * self.speed;
        let sin_a = self.angle.sin() * dt;
        let cos_a = self.angle.cos() * dt;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.x += cos_a;
            self.y += sin_a;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.x += sin_a;
            self.y -= cos_a;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.x -= cos_a;
            self.y -= sin_a;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.x -= sin_a;
            self.y += cos_a;
        }

        if is_key_down(KeyCode::Q) {
            self.angle -= get_frame_time();
        }
        if is_key_down(KeyCode::E) {
            self.angle += get_frame_time();
        }
    }
}
