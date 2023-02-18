use crate::util::*;
use crate::GRID_SIZE;
use bevy::{
    prelude::*,
    render::mesh::shape::{Circle, Quad},
    sprite::MaterialMesh2dBundle,
};
use std::f32::consts::PI;

#[derive(Component)]
struct Ship {}

#[derive(Component)]
struct ShipChair {}

#[derive(Component)]
struct ShipWall {}

#[derive(Component)]
struct ShipEngine {}

#[derive(Resource, Default)]
struct WallStart {
    pos: Vec2,
}

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ship_chair);
        app.add_system(ship_wall_system);
        app.insert_resource(WallStart::default());
    }
}

fn setup_ship_chair(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            Ship {},
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ShipChair {},
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(8.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(Vec3::new(
                        GRID_SIZE / 2.,
                        GRID_SIZE / 2.,
                        0.,
                    )),
                    ..default()
                },
            ));
        });
}

fn ship_wall_system(
    mut commands: Commands,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut wall_start: ResMut<WallStart>,
    ships: Query<Entity, With<Ship>>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    if let Some(cursor_global) = get_cursor_position(windows, camera) {
        let cursor = round_to_grid(cursor_global, GRID_SIZE);

        if buttons.just_pressed(MouseButton::Left) {
            wall_start.pos = cursor;
        }

        if cursor != wall_start.pos {
            println!("from: {} to: {}", wall_start.pos, cursor);
            let mut transform = Transform::from_translation(cursor.extend(0.));
            if cursor.x == wall_start.pos.x {
                transform.rotate_z(PI / 2.);
            }
            if cursor.x > wall_start.pos.x {
                transform.translation += Vec3::new(-GRID_SIZE / 2., 0., 0.);
            }
            if cursor.x < wall_start.pos.x {
                transform.translation += Vec3::new(GRID_SIZE / 2., 0., 0.);
            }
            if cursor.y > wall_start.pos.y {
                transform.translation += Vec3::new(0., -GRID_SIZE / 2., 0.);
            }
            if cursor.y < wall_start.pos.y {
                transform.translation += Vec3::new(0., GRID_SIZE / 2., 0.);
            }
            let ship = ships.single();
            commands.get_entity(ship).unwrap().add_children(|parent| {
                parent.spawn((
                    ShipWall {},
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Quad::new(Vec2::new(GRID_SIZE, 2.)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: transform,
                        ..default()
                    },
                ));
            });
            wall_start.pos = cursor;
        }
    }
}
