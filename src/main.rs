pub use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

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

    app.add_startup_system(setup_background);
    app.run();
}

fn setup_background(
    mut commands: Commands,
    mut grid: ResMut<Assets<GridMaterial>>,
    mut background: ResMut<Assets<BackgroundMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mesh: Mesh2dHandle = meshes.add(Mesh::from(shape::Quad::default())).into();
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(WIDTH, HEIGHT, 1.0),
            ..Default::default()
        },
        material: background.add(BackgroundMaterial {}),
        ..Default::default()
    });
    commands.spawn((MaterialMesh2dBundle {
        mesh,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(WIDTH, HEIGHT, 1.0),
            ..Default::default()
        },
        material: grid.add(GridMaterial {}),
        ..Default::default()
    },));
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {}

impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "background.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "c5ff85a8-583f-41a4-b4a4-0c579e8a8811"]
struct GridMaterial {}

impl Material2d for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        "grid.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "grid.wgsl".into()
    }
}
