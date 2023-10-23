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
use packages::this::{components::*, messages::JumperInput};

const PIXEL_SCALE: f32 = 3.;

#[element_component]
fn SpriteApp(hooks: &mut Hooks) -> Element {
    // let screen_size = entity::get_component(entity::resources(), window_logical_size()).unwrap();
    let sprs = ambient_api::element::use_query(hooks, (spr_path(), spr_pos(), spr_size(), spr_z()));
    // let fsize = screen_size.y as f32 * 0.04;
    Group::el(sprs.iter().map(move |(plr, (path, pos, size, z))| {
        ImageFromUrl {
            url: assets::url(path),
        }
        .el()
        .with(translation(), (pos.as_vec2() * PIXEL_SCALE).extend(*z))
        .with(width(), size.x * PIXEL_SCALE)
        .with(height(), size.y * PIXEL_SCALE)
    }))
}

#[main]
pub fn main() {
    SpriteApp.el().spawn_interactive();
    // AppFromImageUIExample.el().spawn_interactive();
    ambient_api::core::messages::Frame::subscribe(|_| {
        let (delta, input) = input::get_delta();
        JumperInput {
            pinjumppressed: delta.keys.contains(&KeyCode::Space),
            pinx: match (
                input.keys.contains(&KeyCode::Left),
                input.keys.contains(&KeyCode::Right),
            ) {
                (true, false) => -1.,
                (false, true) => 1.,
                _ => 0.,
            },
        }
        .send_server_reliable();
    });
}

#[element_component]
fn AppFromImageUIExample(_hooks: &mut Hooks) -> Element {
    Group::el([FlowColumn::el([
        ImageFromUrl {
            url: assets::url("brick.png"),
        }
        .el()
        .with(size_from_background_image(), ()),
        FlowRow::el([
            ImageFromUrl {
                url: "https://commons.wikimedia.org/w/index.php?title=Special:Redirect/file/Bucephala-albeola-010.jpg"
                    .to_string(),
            }
            .el(),
            ImageFromUrl {
                url: "https://commons.wikimedia.org/w/index.php?title=Special:Redirect/file/Alpha_transparency_image.png"
                    .to_string(),
            }
            .el()
            .with(background_color(), vec4(1., 0., 1., 1.)),
            ImageFromUrl {
                url: "https://commons.wikimedia.org/w/index.php?title=Special:Redirect/file/Bucephala-albeola-010.jpg"
                    .to_string(),
            }
            .el()
            .with(border_radius(), Vec4::ONE * 10.)
            .with(border_color(), vec4(0., 1., 0., 1.))
            .with(border_thickness(), 10.),
            ImageFromUrl {
                url: "invalid url".to_string(),
            }
            .el(),
        ]).with(space_between_items(), 10.)
    ])
    .with(space_between_items(), 10.)
    .with_padding_even(STREET)])
}
