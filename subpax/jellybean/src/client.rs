use ambient_api::prelude::*;

use packages::this::components::*;

#[main]
pub fn main() {
    jellybeans_move_to_collision();
}

fn jellybeans_move_to_collision() {
    let jellybeans_query = query((
        jellybean_pos(),
        jellybean_hitbox(),
        jellybean_vel(),
        jellybean_subpos(),
    ))
    .build();
    let jellybean_solids_query = query((jellybean_pos(), jellybean_hitbox()))
        .requires(jellybean_is_solid())
        .build();
    ambient_api::core::messages::Frame::subscribe(move |_| {
        let beans = jellybeans_query.evaluate();
        let solids = jellybean_solids_query.evaluate();
        for (jellybean, (pos, hitbox, vel, subpos)) in beans {
            let mut pos2: IVec2 = pos;
            let mut vel2: Vec2 = vel;
            let mut subpos2: Vec2 = subpos;
            subpos2 += vel2;
            while (vel2.x > 0. && subpos2.x > 0.) || (subpos2.x >= 1.) {
                if !try_move(&mut pos2, &mut subpos2, ivec2(1, 0), hitbox, &solids) {
                    vel2.x = 0.;
                    subpos2.x = 0.;
                    break;
                }
            }
            while (vel2.y > 0. && subpos2.y > 0.) || (subpos2.y >= 1.) {
                if !try_move(&mut pos2, &mut subpos2, ivec2(0, 1), hitbox, &solids) {
                    vel2.y = 0.;
                    subpos2.y = 0.;
                    break;
                }
            }
            while (vel2.x < 0. && subpos2.x < 0.) || (subpos2.x <= -1.) {
                if !try_move(&mut pos2, &mut subpos2, ivec2(-1, 0), hitbox, &solids) {
                    vel2.x = 0.;
                    subpos2.x = 0.;
                    break;
                }
            }
            while (vel2.y < 0. && subpos2.y < 0.) || (subpos2.y <= -1.) {
                if !try_move(&mut pos2, &mut subpos2, ivec2(0, -1), hitbox, &solids) {
                    vel2.y = 0.;
                    subpos2.y = 0.;
                    break;
                }
            }
            entity::set_component_if_changed(jellybean, jellybean_pos(), pos2);
            entity::set_component_if_changed(jellybean, jellybean_vel(), vel2);
            entity::set_component_if_changed(jellybean, jellybean_subpos(), subpos2);
        }
    });
}

fn try_move(
    my_pos: &mut IVec2,
    my_subpos: &mut Vec2,
    movement: IVec2,
    my_hitbox: IVec4,
    solids: &Vec<(EntityId, (IVec2, IVec4))>,
) -> bool {
    if movement.x == 0 && movement.y == 0 {
        panic!("called try_move with movement vector 0,0");
    } else {
        *my_pos += movement;
        *my_subpos -= movement.as_vec2();
        for (_solid, (solid_pos, solid_hitbox)) in solids {
            if test_overlap(*my_pos, my_hitbox, *solid_pos, *solid_hitbox) {
                // undo move
                *my_pos -= movement;
                *my_subpos += movement.as_vec2();
                return false;
            }
        }
        true // worked!
    }
}

fn test_overlap(pos1: IVec2, rect1: IVec4, pos2: IVec2, rect2: IVec4) -> bool {
    return pos1.x + rect1.x < pos2.x + rect2.x + rect2.z
        && pos1.x + rect1.x + rect1.z > pos2.x + rect2.x
        && pos1.y + rect1.y < pos2.y + rect2.y + rect2.w
        && pos1.y + rect1.y + rect1.w > pos2.y + rect2.y;
}
