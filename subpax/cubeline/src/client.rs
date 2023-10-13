use ambient_api::{
    core::transform::components::{rotation, scale, translation},
    prelude::*,
};

use packages::this::components::*;

#[main]
pub fn main() {
    common_setup();
}

pub fn common_setup() {
    change_query((cubeline_a(), cubeline_b()))
        .track_change((cubeline_a(), cubeline_b()))
        .bind(|clines| {
            for (cline, (a, b)) in clines {
                let w: f32 = entity::get_component(cline, cubeline_width()).unwrap_or(0.1);
                let (r, s, t) = calculate_rst(a, b, w);
                entity::add_component(cline, translation(), t);
                entity::add_component(cline, rotation(), r);
                entity::add_component(cline, scale(), s);
            }
        });
}

fn calculate_rst(a: Vec3, b: Vec3, width: f32) -> (Quat, Vec3, Vec3) {
    (
        Quat::from_rotation_arc(
            vec3(0., 0., 1.),
            (b - a).try_normalize().unwrap_or(vec3(0., 0., 0.)),
        ),
        vec3(width, width, a.distance(b)),
        (a + b) * 0.5,
    )
}
