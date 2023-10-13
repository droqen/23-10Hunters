use ambient_api::{core::messages::Frame, prelude::*};
use packages::this::messages::LeftStick;
#[main]
pub fn main() {
    Frame::subscribe(|_| {
        let input = input::get();
        let mut stick: Vec2 = Vec2::new(0., 0.);
        if input.keys.contains(&KeyCode::Left) {
            stick.x -= 1.;
        }
        if input.keys.contains(&KeyCode::Right) {
            stick.x += 1.;
        }
        if input.keys.contains(&KeyCode::Up) {
            stick.y -= 1.;
        }
        if input.keys.contains(&KeyCode::Down) {
            stick.y += 1.;
        }
        LeftStick { left_stick: stick }.send_server_unreliable();
    });
}
