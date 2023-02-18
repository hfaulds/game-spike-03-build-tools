mod background;
mod build;
mod util;

use background::{setup_background, BackgroundMaterial, GridMaterial};
use bevy::{prelude::*, sprite::Material2dPlugin};
use build::BuildPlugin;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 800.0;
pub const GRID_SIZE: f32 = 20.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "game-spike-02-shaders".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        },
        ..Default::default()
    }));
    app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default());
    app.add_plugin(Material2dPlugin::<GridMaterial>::default());

    app.add_plugin(BuildPlugin);

    app.add_startup_system(setup_background);

    app.run();
}
