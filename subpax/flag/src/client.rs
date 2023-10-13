use ambient_api::{core::transform::components::translation, prelude::*};

use packages::this::components::*;

#[main]
pub fn main() {
    common_setup();
}

pub fn common_setup() {
    spawn_query(flagpole()).bind(|flags| {
        for (flag, pole) in flags {
            if let Some(pole_position) = entity::get_component(pole, translation()) {
                if !entity::has_component(flag, translation()) {
                    entity::add_component(flag, translation(), pole_position);
                }
            } else {
                println!("Error - Invalid flagpole '{pole:?}' (missing translation component)");
            }
        }
    });

    query((flagpole(), translation())).each_frame(|flags| {
        for (flag, (pole, pos)) in flags {
            let (lerp_toward, lin_toward, mut vel, inertia) = (
                entity::get_component(flag, flaglerp()).unwrap_or(0.),
                entity::get_component(flag, flaglin()).unwrap_or(0.),
                entity::get_component(flag, flag_velocity()).unwrap_or(Vec3::ZERO),
                entity::get_component(flag, flaginertia()).unwrap_or(0.),
            );

            if lerp_toward > 0. || lin_toward > 0. {
                let (arms_length, target_offset) = (
                    entity::get_component(flag, flagpole_arms_length()).unwrap_or(0.),
                    entity::get_component(flag, flagpole_target_offset()).unwrap_or(Vec3::ZERO),
                );
                let mut target_pos = entity::get_component(pole, translation()).unwrap();
                target_pos += target_offset;
                if arms_length > 0. {
                    let dir = match (target_pos - pos).try_normalize() {
                        None => vec3(0., 0., 1.),
                        Some(dir) => dir,
                    };
                    target_pos -= dir * arms_length;
                }
                let mut pos2 = pos;
                let dist_sqr: f32 = target_pos.distance_squared(pos);
                if dist_sqr <= lin_toward * lin_toward {
                    pos2 = target_pos;
                } else {
                    let dir = (target_pos - pos).normalize();
                    pos2 += dir * lin_toward;
                    pos2 = pos2.lerp(target_pos, lerp_toward);
                }

                vel = vel.lerp(pos2 - pos, 1. - inertia);
            } else {
                vel *= 1. - inertia;
            }

            entity::set_component(flag, translation(), pos + vel);
            entity::add_component(flag, flag_velocity(), vel);
        }
    });
}
