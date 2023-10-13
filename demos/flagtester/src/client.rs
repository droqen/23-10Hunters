use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::{cube, quad},
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

#[main]
pub fn main() {
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

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    // make a droplet with cubeline and flags
    // todo

    use packages::cubeline::components::{cubeline_a, cubeline_b, cubeline_width};
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
    use packages::flag::components::{flaginertia, flaglerp, flagpole};
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
