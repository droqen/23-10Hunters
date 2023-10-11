use ambient_api::{
    core::{
        primitives::components::cube,
        transform::components::{rotation, scale, translation},
    },
    prelude::*,
};

use packages::{droplet::components::*, this::components::*};

#[main]
pub fn main() {
    def_my_inpts();
    def_sparks();
}

fn def_my_inpts() {
    ambient_api::core::messages::Frame::subscribe(|_| {
        let (delta, input) = input::get_delta();
        if delta.keys.contains(&KeyCode::R) {
            spawn_spark();
            spawn_spark();
            spawn_spark();
        }
    });
}

fn def_sparks() {
    spawn_query(())
        .requires((loop_index(), is_spark()))
        .bind(|sparks| {
            dbg!(&sparks);
            for (spark, _) in sparks {
                entity::despawn(spark);
            }
        });
}

fn spawn_spark() {
    Entity::new()
        .with(is_spark(), ())
        .with(loop_length(), rr(0.5, 1.))
        .with(tween_atob_a(), vec3(0., 0., 0.))
        .with(tween_atob_b(), vec3(rr(-2., 2.), rr(-2., 2.), 5.))
        .with(cube(), ())
        .with(
            rotation(),
            Quat::from_euler(
                glam::EulerRot::XYZ,
                rr(0., 6.28),
                rr(0., 6.28),
                rr(0., 6.28),
            ),
        )
        .with(scale(), Vec3::splat(rr(0.1, 0.2)))
        .spawn();
}

fn rr(a: f32, b: f32) -> f32 {
    a + random::<f32>() * (b - a)
}
