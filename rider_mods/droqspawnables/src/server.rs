use ambient_api::{
    core::{
        physics::components::sphere_collider,
        primitives::concepts::Sphere,
        rendering::components::color,
        transform::components::{rotation, scale, translation},
    },
    prelude::*,
};
use packages::tangent_rider_schema::concepts::Spawnable;

#[main]
pub fn main() {
    {
        let base = Entity::new()
            .with_merge(
                Sphere {
                    sphere_radius: 1.,
                    sphere_sectors: 8,
                    sphere_stacks: 4,
                    ..Sphere::suggested()
                }
                .make(),
            )
            // Hiding it under the map shouldn't be necessary, but there's no easy fix for this at present
            .with(translation(), Vec3::Z * -100.);

        Spawnable {
            spawnable_name: "Sphere".to_string(),
            spawnable_cost: 300,
            spawnable_main_ref: base.clone().with(sphere_collider(), 1.).spawn(),
            spawnable_ghost_ref: base.spawn(),
        }
        .spawn();
    }
}
