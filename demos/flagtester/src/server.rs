use ambient_api::{
    cb::Callback,
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        messages::Frame,
        primitives::{
            components::{cube, quad},
            concepts::Sphere,
        },
        transform::{
            components::{lookat_target, translation},
            concepts::Transformable,
        },
    },
    prelude::*,
};
use packages::{
    cubeline::components::{cubeline_a, cubeline_b, cubeline_width},
    flag::components::{flaginertia, flaglerp, flaglin, flagpole, flagpole_target_offset},
};

#[main]
pub fn main() {
    spawn_cam();
    // spawn_quad();
    // spawn_test1_simple();
    spawn_test2_orb_body();
}

fn spawn_cam() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();
}

fn spawn_quad() {
    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();
}

fn spawn_test1_simple() {
    let tester_cline = Entity::new()
        .with(cube(), ())
        .with(cubeline_a(), vec3(0., 0., 5.))
        .with(cubeline_b(), vec3(0., 0., 0.))
        .with(cubeline_width(), 0.1)
        .spawn();
    let wobbler = Entity::new()
        .with(cube(), ())
        .with(translation(), vec3(0., 0., 0.))
        .spawn();
    let wobbler_flag = Entity::new()
        .with(cube(), ())
        .with(flagpole(), wobbler)
        .with(flaglerp(), 0.1)
        .with(flaginertia(), 0.1)
        .spawn();
    ambient_api::core::messages::Frame::subscribe(move |_| {
        entity::mutate_component(wobbler, translation(), |pos| {
            pos.x = game_time().as_secs_f32().sin() * 4.;
            pos.y = (game_time().as_secs_f32() * 0.2).cos() * 2.;
        });
        entity::set_component(
            tester_cline,
            cubeline_b(),
            entity::get_component(wobbler, translation()).unwrap(),
        );
    });
}

fn spawn_test2_orb_body() {
    let ball_goto = Entity::new().with(translation(), vec3(10., 0., 0.)).spawn();
    let ball_head = make_ball(0.5)
        .with(translation(), vec3(0., 0., 1.))
        .with(flagpole(), ball_goto)
        .with(flagpole_target_offset(), vec3(0., 0., 1.))
        .with(flaglin(), 0.05)
        .with(flaglerp(), 0.01)
        .with(flaginertia(), 0.1)
        .spawn();
    let ball_body = make_ball(0.2)
        .with(flagpole(), ball_head)
        .with(flagpole_target_offset(), vec3(0., 0., -1.))
        .with(flaglin(), 0.02)
        .with(flaglerp(), 0.1)
        .with(flaginertia(), 0.1)
        .spawn();
    let ball_line = Entity::new()
        .with(cube(), ())
        .with(cubeline_width(), 0.1)
        .spawn();
    Frame::subscribe(move |_| {
        entity::mutate_component(ball_head, translation(), |pos| {
            pos.x += 0.01;
        });
        entity::add_components(
            ball_line,
            Entity::new()
                .with(cubeline_a(), pos_of(ball_head))
                .with(cubeline_b(), pos_of(ball_body)),
        );
        if random::<f32>() < 0.05 {
            entity::add_component(
                ball_goto,
                translation(),
                vec3(random::<f32>() * 10. - 5., random::<f32>() * 10. - 5., 0.),
            );
        }
    });
}

fn make_ball(radius: f32) -> Entity {
    Entity::new()
        .with_merge(Sphere {
            sphere: (),
            sphere_radius: radius,
            sphere_sectors: 12,
            sphere_stacks: 4,
        })
        .with(translation(), vec3(0., 0., 0.))
}

fn pos_of(entity: EntityId) -> Vec3 {
    entity::get_component(entity, translation()).unwrap_or(Vec3::ZERO)
}
