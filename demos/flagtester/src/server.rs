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

    use packages::cubeline::components::{cubeline_a, cubeline_b};
    let tester_cline = Entity::new()
        .with(cube(), ())
        .with(cubeline_a(), vec3(0., 0., 0.))
        .with(cubeline_b(), vec3(10., 0., 0.))
        .spawn();
    ambient_api::core::messages::Frame::subscribe(move |_| {
        entity::mutate_component(tester_cline, cubeline_b(), |b| {
            if random::<f32>() < 0.1 {
                *b = vec3(
                    random::<f32>() - 0.5,
                    random::<f32>() - 0.5,
                    random::<f32>() - 0.5,
                ) * 20.;
            } else {
                *b *= 0.99;
            }
        });
    });
}