use crate::{HEIGHT, WIDTH};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn setup_background(
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
            translation: Vec3::new(0.0, 0.0, 0.1),
            scale: Vec3::new(WIDTH, HEIGHT, 1.0),
            ..Default::default()
        },
        material: grid.add(GridMaterial {}),
        ..Default::default()
    },));
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
pub struct BackgroundMaterial {}

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
pub struct GridMaterial {}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "grid.wgsl".into()
    }
}
