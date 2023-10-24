use ambient_api::prelude::*;

use packages::{jellybean::components::*, pixie::components::*, this::components::*};

#[main]
pub fn main() {
    let playerbean = Entity::new()
        .with(pix_sprurl(), packages::this::assets::url("player10.png"))
        .with(pix_size(), ivec2(10, 10))
        .with(pix_z(), 0)
        .with(jellybean_pos(), ivec2(13, 17) * 10)
        .with(jellybean_hitbox(), ivec4(2, 4, 6, 6))
        .with(jellybean_vel(), vec2(0., 0.))
        .with(jellybean_subpos(), vec2(0., 0.))
        .with(jellybean_casts_down(), ())
        .with(jellybean_casts_right(), ())
        .with(jellybean_casts_left(), ())
        .with(prunanim(), 0)
        .with(pfaceleft(), false)
        .with(pflorbuf(), 0)
        .with(pjumpbuf(), 0)
        .with(pwalljumped(), 0)
        .spawn();
    change_query(jellybean_pos())
        .track_change(jellybean_pos())
        .requires(pix_sprurl())
        .bind(|jellypixies| {
            for (jp, pos) in jellypixies {
                entity::add_component(jp, pix_place(), pos);
            }
        });

    let PLAYER_SPR_R0 = packages::this::assets::url("pR10.png");
    let PLAYER_SPR_R1 = packages::this::assets::url("pRUp10.png");
    let PLAYER_SPR_L0 = packages::this::assets::url("pL10.png");
    let PLAYER_SPR_L1 = packages::this::assets::url("pLUp10.png");

    ambient_api::core::messages::Frame::subscribe(move |_| {
        let (delta, input) = input::get_delta();
        let pin_jump = delta.keys.contains(&KeyCode::Space);
        let pin_jumpheld = input.keys.contains(&KeyCode::Space);
        let pin_dx = match (
            input.keys.contains(&KeyCode::Left),
            input.keys.contains(&KeyCode::Right),
        ) {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };

        entity::mutate_component(playerbean, pflorbuf(), |florbuf| {
            if *florbuf > 0 {
                *florbuf -= 1
            }
        });

        entity::mutate_component(playerbean, pjumpbuf(), |jumpbuf| {
            if *jumpbuf > 0 {
                *jumpbuf -= 1
            }
        });

        let player_floored =
            entity::get_component(playerbean, jellybean_touching_down()).unwrap_or(false);

        if player_floored {
            entity::add_component(playerbean, pflorbuf(), 4);
            entity::add_component(playerbean, pwalljumped(), 0);
        }

        if pin_jump {
            entity::add_component(playerbean, pjumpbuf(), 4);
        }

        let walljump_dir = entity::get_component(playerbean, pwalljumped()).unwrap_or(0);

        entity::mutate_component(playerbean, jellybean_vel(), move |vel| {
            if walljump_dir == 0 {
                vel.x = tow(vel.x, pin_dx * 1.5, 0.1);
            } else {
                vel.x = tow(vel.x, walljump_dir as f32 * 0.5, 0.01);
                if vel.x.abs() <= 0.55 || vel.y > 0. {
                    entity::add_component(playerbean, pwalljumped(), 0);
                }
            }
            vel.y += 0.13;
            if vel.y < 0. && !pin_jumpheld {
                vel.y += 0.15; // more gravity - fast fall on release
            }

            let florbuf = entity::get_component(playerbean, pflorbuf()).unwrap_or(0);
            let jumpbuf = entity::get_component(playerbean, pjumpbuf()).unwrap_or(0);

            if jumpbuf > 0 && florbuf > 0 {
                vel.y = -3.; // jump height
                entity::set_component(playerbean, pjumpbuf(), 0);
                entity::set_component(playerbean, pflorbuf(), 0);
            } else if jumpbuf > 0 {
                if entity::get_component(playerbean, jellybean_touching_left()).unwrap_or_default()
                {
                    vel.x = 1.5;
                    vel.y = -2.; // walljump height
                    entity::add_component(playerbean, pwalljumped(), 1);
                    entity::set_component(playerbean, pjumpbuf(), 0);
                } else if entity::get_component(playerbean, jellybean_touching_right())
                    .unwrap_or_default()
                {
                    vel.x = -1.5;
                    vel.y = -2.; // walljump height
                    entity::add_component(playerbean, pwalljumped(), -1);
                    entity::set_component(playerbean, pjumpbuf(), 0);
                }
            }

            if player_floored && pin_dx != 0. {
                entity::mutate_component(playerbean, prunanim(), |run| *run = (*run + 1) % 16);
            } else if player_floored {
                entity::mutate_component(playerbean, prunanim(), |run| *run = 0);
            } else {
                entity::mutate_component(playerbean, prunanim(), |run| *run = 1);
            }

            if walljump_dir != 0 {
                entity::add_component(playerbean, pfaceleft(), walljump_dir < 0);
            } else if pin_dx != 0. {
                entity::add_component(playerbean, pfaceleft(), pin_dx < 0.);
            }
        });

        let player_runanim = entity::get_component(playerbean, prunanim()).unwrap();
        let player_faceleft = entity::get_component(playerbean, pfaceleft()).unwrap();
        entity::set_component_if_changed(
            playerbean,
            pix_sprurl(),
            match (player_runanim > 0 && player_runanim < 8, player_faceleft) {
                (false, true) => PLAYER_SPR_L0.clone(),
                (true, true) => PLAYER_SPR_L1.clone(),
                (false, false) => PLAYER_SPR_R0.clone(),
                (true, false) => PLAYER_SPR_R1.clone(),
            },
        );
    });

    for tx in 8..20 {
        for ty in 3..19 {
            if {
                match ty {
                    18 => true,
                    17 => false,
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

fn tow(a: f32, b: f32, rate: f32) -> f32 {
    if a + rate < b {
        a + rate
    } else if a - rate > b {
        a - rate
    } else {
        b
    }
}
