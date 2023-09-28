use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        model::components::model_from_url,
        primitives::components::quad,
        transform::components::{lookat_target, rotation, translation},
    },
    prelude::*,
};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 2.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 1.))
    .spawn();

    let model = Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(rotation(), Quat::IDENTITY)
        .with(
            model_from_url(),
            packages::this::assets::url("chicken_fbx_to_glb.glb"),
            // packages::this::assets::url("Muscle Chicken with T-Pose.fbx"),
            // packages::this::assets::url("Muscle Chicken.fbx"),
        )
        .spawn();

    ambient_api::core::messages::Frame::subscribe(move |_| {
        entity::mutate_component(model, rotation(), |rot| {
            *rot = Quat::from_rotation_z(delta_time()) * *rot
        });
    });

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient!");
}
