use ambient_api::{
    core::player::components::{is_player, user_id},
    prelude::*,
};

use packages::this::{components::*, messages::JumperInput};

#[main]
pub fn main() {
    const XMAX: i32 = 24;
    const YMAX: i32 = 18;
    let mut free_spaces: Vec<(i32, i32)> = vec![];
    for cx in 0..XMAX {
        for cy in 0..YMAX {
            if cx == 0 || cx == XMAX - 1 || cy == 0 || cy == YMAX - 1 || random::<f32>() < 0.357 {
                spawn_brick(cx * 10, cy * 10);
            } else {
                free_spaces.push((cx, cy));
            }
        }
    }
    spawn_query((is_player(), user_id())).bind(move |plrs| {
        for (plr, (_, plr_uid)) in plrs {
            let free_space_index = random::<usize>() % free_spaces.len();
            spawn_player(
                free_spaces[free_space_index].0 * 10,
                free_spaces[free_space_index].1 * 10,
                plr_uid,
            );
        }
    });
    let findmoverplayers = query((user_id(), spr_vel())).build();
    JumperInput::subscribe(move |ctx, msg| {
        for (plr, (plr_uid, vel)) in findmoverplayers.evaluate() {
            if plr_uid == ctx.client_user_id().unwrap() {
                let mut newvel: Vec2 = vel;
                newvel.x = newvel.x * 0.9 + 0.1 * msg.pinx;
                if msg.pinjumppressed {
                    newvel.y = -2.;
                }
                entity::set_component(plr, spr_vel(), newvel);
            }
        }
    });
    let findbricks = query((spr_pos(), spr_boxrect()))
        .requires(is_brick())
        .build();
    query((spr_pos(), spr_subpos(), spr_vel(), spr_boxrect())).each_frame(move |movingsprs| {
        for (spr, (pos, subpos, vel, rect)) in movingsprs {
            let mut subpos2: Vec2 = subpos;
            let mut pos2: IVec2 = pos;
            let mut vel2: Vec2 = vel;

            vel2.y += 0.05; // gravity

            let bricks = findbricks.evaluate();

            fn any_brick_at(
                pos2: IVec2,
                rect: Vec4,
                bricks: Vec<(EntityId, (IVec2, Vec4))>,
            ) -> bool {
                for (brick, (brickpos, brickrect)) in bricks {
                    if test_overlap(brickpos.as_vec2(), brickrect, pos2.as_vec2(), rect) {
                        return true;
                    };
                }
                return false;
            }

            // for x
            subpos2.x += vel.x;
            while subpos2.x > 0.5 {
                pos2.x += 1;
                subpos2.x -= 1.;
                if any_brick_at(pos2, rect, bricks.clone()) {
                    pos2.x -= 1;
                    subpos2.x = 0.;
                    vel2.x = 0.;
                }
            }
            while subpos2.x < -0.5 {
                pos2.x -= 1;
                subpos2.x += 1.;
                if any_brick_at(pos2, rect, bricks.clone()) {
                    pos2.x += 1;
                    subpos2.x = 0.;
                    vel2.x = 0.;
                }
            }

            // for y
            subpos2.y += vel.y;
            while subpos2.y > 0.5 {
                pos2.y += 1;
                subpos2.y -= 1.;
                if any_brick_at(pos2, rect, bricks.clone()) {
                    pos2.y -= 1;
                    subpos2.y = 0.;
                    vel2.y = 0.;
                }
            }
            while subpos2.y < -0.5 {
                pos2.y -= 1;
                subpos2.y += 1.;
                if any_brick_at(pos2, rect, bricks.clone()) {
                    pos2.y += 1;
                    subpos2.y = 0.;
                    vel2.y = 0.;
                }
            }

            entity::set_component(spr, spr_pos(), pos2);
            entity::set_component(spr, spr_subpos(), subpos2);
            entity::set_component(spr, spr_vel(), vel2);
        }
    });
}

const TILE_SIZE: Vec2 = Vec2::new(10., 10.);
const PIXEL_SCALE: f32 = 1.;

fn spawn_brick(x: i32, y: i32) -> EntityId {
    Entity::new()
        .with(spr_pos(), ivec2(x, y))
        .with(spr_z(), 0.0)
        .with(spr_size(), TILE_SIZE * PIXEL_SCALE) // hm. 3x size
        .with(spr_path(), "brick10.png".into())
        .with(spr_boxrect(), vec4(0., 0., 10., 10.) * PIXEL_SCALE)
        .with(is_brick(), ())
        .spawn()
}

fn spawn_player(x: i32, y: i32, uid: String) -> EntityId {
    Entity::new()
        .with(spr_pos(), ivec2(x, y))
        .with(spr_z(), 0.1)
        .with(spr_subpos(), vec2(0., 0.))
        .with(spr_size(), TILE_SIZE * PIXEL_SCALE) // hm. 3x size
        .with(spr_path(), "player10.png".into())
        .with(spr_boxrect(), vec4(2., 5., 6., 5.) * PIXEL_SCALE)
        .with(spr_vel(), vec2(0., 0.))
        .with(spr_playercontrolled(), ())
        .with(user_id(), uid)
        .spawn()
}

fn test_overlap(pos1: Vec2, rect1: Vec4, pos2: Vec2, rect2: Vec4) -> bool {
    return (pos1.x + rect1.x < pos2.x + rect2.x + rect2.z
        && pos1.x + rect1.x + rect1.z > pos2.x + rect2.x
        && pos1.y + rect1.y < pos2.y + rect2.y + rect2.w
        && pos1.y + rect1.y + rect1.w > pos2.y + rect2.y);
}
