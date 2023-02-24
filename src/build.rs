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
    from: Option<Vec2>,
}

struct WallCreateEvent {
    from: Vec2,
    to: Vec2,
    dir: Vec2,
}

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WallCreateEvent>()
            .add_startup_system(setup_ship_chair)
            .add_system(place_wall_system)
            .add_system(spawn_wall_system)
            .insert_resource(WallStart::default());
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

fn place_wall_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut wall_start: ResMut<WallStart>,
    mut events: EventWriter<WallCreateEvent>,
) {
    match wall_start.from {
        Some(from) => {
            if buttons.pressed(MouseButton::Left) {}
            if buttons.just_released(MouseButton::Left) {
                if let Some(cursor_global) = get_cursor_position(windows, camera) {
                    let to = round_to_grid(cursor_global, GRID_SIZE);

                    let dist = to - from;
                    let direction = *[
                        Vec2::new(0.0, 1.0),
                        Vec2::new(1.0, 1.0),
                        Vec2::new(1.0, 0.0),
                        Vec2::new(1.0, -1.0),
                        Vec2::new(0.0, -1.0),
                        Vec2::new(-1.0, -1.0),
                        Vec2::new(-1.0, 0.0),
                        Vec2::new(-1.0, 1.0),
                    ]
                    .iter()
                    .min_by(|x, y| {
                        x.normalize()
                            .distance(dist.normalize())
                            .partial_cmp(&y.normalize().distance(dist.normalize()))
                            .unwrap()
                    })
                    .unwrap();

                    for i in 0..((dist / direction / GRID_SIZE).min_element().floor() as usize) {
                        let a = from + (i as f32 * direction) * GRID_SIZE;
                        let b = from + ((i + 1) as f32 * direction) * GRID_SIZE;
                        events.send(WallCreateEvent {
                            from: a,
                            to: b,
                            dir: direction,
                        });
                    }
                    wall_start.from = None;
                }
            }
        }
        None => {
            if buttons.just_pressed(MouseButton::Left) {
                if let Some(cursor_global) = get_cursor_position(windows, camera) {
                    wall_start.from = Some(round_to_grid(cursor_global, GRID_SIZE));
                }
            }
        }
    }
}

fn spawn_wall_system(
    mut commands: Commands,
    ships: Query<Entity, With<Ship>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<WallCreateEvent>,
) {
    for event in events.iter() {
        let to = event.to;
        let from = event.from;
        let dir = event.dir;
        let mut size = Vec2::new(GRID_SIZE, 2.);

        let mut transform = Transform::from_translation(to.extend(0.));
        if to.x == from.x {
            transform.rotate_z(PI / 2.);
        }
        if to.x != from.x && to.y != from.y {
            if dir.x == dir.y {
                transform.rotate_z(PI / 4.0);
            } else {
                transform.rotate_z(PI / -4.0);
            }
            size = Vec2::new(GRID_SIZE * std::f32::consts::SQRT_2, 2.);
        }
        if to.x > from.x {
            transform.translation += Vec3::new(-GRID_SIZE / 2., 0., 0.);
        }
        if to.x < from.x {
            transform.translation += Vec3::new(GRID_SIZE / 2., 0., 0.);
        }
        if to.y > from.y {
            transform.translation += Vec3::new(0., -GRID_SIZE / 2., 0.);
        }
        if to.y < from.y {
            transform.translation += Vec3::new(0., GRID_SIZE / 2., 0.);
        }
        let ship = ships.single();
        commands.get_entity(ship).unwrap().add_children(|parent| {
            parent.spawn((
                ShipWall {},
                MaterialMesh2dBundle {
                    mesh: meshes.add(Quad::new(size).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: transform,
                    ..default()
                },
            ));
        });
    }
}
