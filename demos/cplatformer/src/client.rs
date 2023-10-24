use ambient_api::prelude::*;

use packages::{jellybean::components::*, pixie::components::*};

#[main]
pub fn main() {
    let playerbean = Entity::new()
        .with(pix_sprurl(), packages::this::assets::url("player10.png"))
        .with(pix_size(), ivec2(10, 10))
        .with(pix_z(), 0)
        .with(jellybean_pos(), ivec2(4, 8) * 10)
        .with(jellybean_hitbox(), ivec4(3, 3, 4, 7))
        .with(jellybean_vel(), vec2(0., 0.))
        .with(jellybean_subpos(), vec2(0., 0.))
        .spawn();
    change_query(jellybean_pos())
        .track_change(jellybean_pos())
        .requires(pix_sprurl())
        .bind(|jellypixies| {
            for (jp, pos) in jellypixies {
                entity::add_component(jp, pix_place(), pos);
            }
        });
    ambient_api::core::messages::Frame::subscribe(move |_| {
        let (delta, input) = input::get_delta();
        let pin_jump = delta.keys.contains(&KeyCode::Space);
        let pin_dx = match (
            input.keys.contains(&KeyCode::Left),
            input.keys.contains(&KeyCode::Right),
        ) {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };
        entity::mutate_component(playerbean, jellybean_vel(), |vel| {
            vel.x = vel.x * 0.9 + pin_dx * 0.1;
            vel.y += 0.05;
            if pin_jump {
                vel.y = -1.;
            }
        });
    });

    for tx in 0..10 {
        for ty in 0..10 {
            if {
                match ty {
                    9 => true,
                    8 => false,
                    _ => random::<f32>() < 0.357,
                }
            } {
                Entity::new()
                    .with(pix_sprurl(), packages::this::assets::url("brick10.png"))
                    .with(pix_size(), ivec2(10, 10))
                    .with(pix_z(), 1)
                    .with(jellybean_pos(), ivec2(tx, ty) * 10)
                    .with(jellybean_hitbox(), ivec4(0, 0, 10, 10))
                    .with(jellybean_is_solid(), ())
                    .spawn();
            }
        }
    }
}
