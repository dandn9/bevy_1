use bevy::{math::vec2, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::MassProperties};

const NUM_FLOOR_TILES: i32 = 30;
const TILE_HEIGHT: f32 = 40.;
const PLAYER_SIZE: f32 = 60.;

fn main() {
    App::new()
        .insert_resource(IsPlayerOnGround(false))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .register_inspectable::<MeshType>()
        .add_startup_system(setup_physics)
        .add_startup_system(setup)
        .add_startup_system(spawn_floor)
        .add_system(handle_input)
        .add_system(is_player_on_ground)
        .run();
}
struct IsPlayerOnGround(bool);

#[derive(Reflect, Clone, Copy, Inspectable, Component)]
enum MeshType {
    Floor,
    Wall,
    Cealing,
}
#[derive(Component)]
struct FloorTile(i32);
#[derive(Component)]
struct Player;

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    /* Create the bouncing ball. */
}

fn is_player_on_ground(
    mut is_played_on_ground: ResMut<IsPlayerOnGround>,
    player_q: Query<(&Collider, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
    mut commands: Commands,
    world: &World
) {
    let player = player_q.get_single().unwrap();

    let player_transform = player.1;

    let ray_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - (PLAYER_SIZE / 2.0) - 2.0,
    );
    let ray_dir = Vec2::new(player_transform.translation.x, ray_pos.y + (-20.));

    let max_toi = 1.0;
    lines.line(
        Vec3::new(ray_pos.x, ray_pos.y, 0.),
        Vec3::new(ray_dir.x, ray_dir.y, 0.),
        1.,
    );

    let solid = true;
    let filter = QueryFilter::default();

    if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = ray_pos + ray_dir * toi;


        // commands.entity(entity)
        // player.cast_local_ray(ray_origin, ray_dir, max_toi, solid)
    }
}

fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut dbg: ResMut<DebugRenderContext>,
    mut player_q: Query<(Entity, &mut Velocity), With<Player>>,
) {
    // print!("ok")
    if keys.just_pressed(KeyCode::F) {
        dbg.enabled = !dbg.enabled;
    }

    let mut player = player_q.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::W) {
        commands.entity(player.0).insert(ExternalImpulse {
            impulse: vec2(0., 220.),
            torque_impulse: 0.,
        });
    }
    if keys.pressed(KeyCode::A) {
        (*player.1).linvel = vec2(-200.0, (*player.1).linvel.y);
    }
    if keys.pressed(KeyCode::D) {
        (*player.1).linvel = vec2(200.0, (*player.1).linvel.y);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: vec2(PLAYER_SIZE, PLAYER_SIZE),
                    flip: false,
                }))
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Player)
        .insert(Name::from("Player"))
        .insert(Collider::cuboid(PLAYER_SIZE / 2., PLAYER_SIZE / 2.))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Mass(1.))
        .insert(Velocity::default())
        .insert(GravityScale(5.0));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_container = commands
        .spawn()
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3 {
                x: (NUM_FLOOR_TILES / 2 * 40 * -1) as f32,
                y: 0.,
                z: 0.,
            }),
            ..default()
        })
        .with_children(|parent| {
            for i in 0..=NUM_FLOOR_TILES {
                parent
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad {
                                size: vec2(TILE_HEIGHT, TILE_HEIGHT),
                                flip: false,
                            }))
                            .into(),
                        transform: Transform::from_xyz((i * 40) as f32, 0., 0.),
                        material: materials.add(ColorMaterial::from(Color::GRAY)),
                        ..default()
                    })
                    .insert(MeshType::Floor)
                    .insert(FloorTile(i));
            }
        })
        .id();

    commands
        .spawn()
        .insert(Name::from("Floor"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(0., -100., 0.),
            ..Default::default()
        })
        .insert(Collider::cuboid(
            (NUM_FLOOR_TILES * 20 + 20) as f32,
            TILE_HEIGHT / 2.,
        ))
        .add_child(mesh_container);
}
