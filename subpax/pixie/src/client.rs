use ambient_api::{
    core::{
        layout::components::space_between_items,
        rect::components::{
            background_color, border_color, border_radius, border_thickness,
            size_from_background_image,
        },
        transform::components::translation,
    },
    prelude::*,
    ui::ImageFromUrl,
};
use packages::this::assets;
use packages::this::components::*;

const PIXEL_SCALE: f32 = 3.;

#[main]
pub fn main() {
    // // test pixie
    // Entity::new()
    //     .with(pix_sprurl(), packages::this::assets::url("player10.png"))
    //     .with(pix_place(), ivec2(8, 8))
    //     .with(pix_size(), ivec2(10, 10))
    //     .with(pix_z(), 0)
    //     .spawn();

    // // test flipped pixie? - no, can't flip
    // Entity::new()
    //     .with(pix_sprurl(), packages::this::assets::url("player10.png"))
    //     .with(pix_place(), ivec2(8, 18))
    //     .with(pix_size(), ivec2(-10, 10))
    //     .with(pix_z(), 0)
    //     .spawn();

    entity::add_component_if_required(packages::this::entity(), pixg_scale(), 2);
    entity::add_component_if_required(packages::this::entity(), pixg_offset(), ivec2(0, 0));
    SpriteApp.el().spawn_interactive();
}

#[element_component]
fn SpriteApp(hooks: &mut Hooks) -> Element {
    // let screen_size = entity::get_component(entity::resources(), window_logical_size()).unwrap();
    let global_pixie_scale: i32 = 3;
    let global_pixie_offset: IVec2 = ivec2(0, 0);
    // let (_, (global_pixie_scale, global_pixie_offset)) =
    //     ambient_api::element::use_query(hooks, (pixg_scale(), pixg_offset()))
    //         .first()
    //         .unwrap_or(&(Default::default(), (1, ivec2(0, 0))));
    let pixies =
        ambient_api::element::use_query(hooks, (pix_sprurl(), pix_place(), pix_size(), pix_z()));
    // let fsize = screen_size.y as f32 * 0.04;
    Group::el(pixies.iter().map(move |(pixie, (sprurl, place, size, z))| {
        ImageFromUrl {
            url: sprurl.clone(),
        }
        .el()
        .with(
            translation(),
            (*place * global_pixie_scale + global_pixie_offset)
                .as_vec2()
                .extend(*z as f32 * 0.002),
        )
        .with(width(), (size.x * global_pixie_scale) as f32)
        .with(height(), (size.y * global_pixie_scale) as f32)
    }))
}
