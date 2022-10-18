use bevy::{math::vec2, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_1::utils::{draw_line, draw_line_colored};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

const NUM_FLOOR_TILES: i32 = 30;
const TILE_HEIGHT: f32 = 40.;
const PLAYER_SIZE: f32 = 60.;

fn main() {
    App::new()
        .insert_resource(CanJump(false))
        .insert_resource(IsOnWall {
            is_on_wall: false,
            side: None,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .register_inspectable::<MeshType>()
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_floor)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, add_debug_view)
        .add_system(is_player_on_ground)
        .add_system(is_player_on_wall)
        .add_system(handle_input.after(is_player_on_ground))
        .run();
}
#[derive(Debug)]
enum WallSide {
    Left,
    Right,
}
#[derive(Debug)]
struct IsOnWall {
    is_on_wall: bool,
    side: Option<WallSide>,
}

#[derive(Debug)]
struct CanJump(bool);

#[derive(Default, Debug)]
struct IsPlayerInFloor(bool);

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

fn is_player_on_wall(
    player_q: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
    mut res_is_on_wall: ResMut<IsOnWall>,
) {
    let player = player_q.get_single().unwrap();
    let player_transform = player;

    let l_ray_pos = Vec2::new(
        player_transform.translation.x - (PLAYER_SIZE / 2.),
        player_transform.translation.y,
    );
    let l_ray_dir = Vec2::new(-1., 0.);
    let max_toi = 10.0;

    draw_line_colored(&l_ray_pos, &l_ray_dir, max_toi, &mut lines, None);
    let r_ray_pos = Vec2::new(
        player_transform.translation.x + (PLAYER_SIZE / 2.),
        player_transform.translation.y,
    );
    let r_ray_dir = Vec2::new(1., 0.);
    let max_toi = 10.0;

    draw_line_colored(&r_ray_pos, &r_ray_dir, max_toi, &mut lines, None);

    let solid = false;

    let filter =
        QueryFilter::new().groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1).into());

    let mut is_on_wall = false;
    let mut side: Option<WallSide> = None;

    if let Some((_entity, _toi)) =
        rapier_context.cast_ray(r_ray_pos, r_ray_dir, max_toi, solid, filter)
    {
        is_on_wall = true;
        side = Some(WallSide::Right);
    } else {
        if let Some((_entity, _toi)) =
            rapier_context.cast_ray(l_ray_pos, l_ray_dir, max_toi, solid, filter)
        {
            is_on_wall = true;
            side = Some(WallSide::Left);
        }
    }

    *res_is_on_wall = IsOnWall { is_on_wall, side };
}

fn is_player_on_ground(
    mut can_jump: ResMut<CanJump>,
    mut is_player_in_floor: Local<IsPlayerInFloor>,
    player_q: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
    lines: ResMut<DebugLines>,
) {
    let player_transform = player_q.get_single().unwrap();

    let shape = Collider::cuboid((PLAYER_SIZE / 2.) - 3., 2.0);
    let shape_pos = Vec2::new(1.0, 2.0);
    let shape_rot = 0.8;
    let shape_vel = Vec2::new(0.1, 0.4);
    let max_toi = 4.0;

    let ray_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - (PLAYER_SIZE / 2.0),
    );
    // let ray_dir = Vec2::new(player_transform.translation.x, ray_pos.y - 20.);
    let ray_dir = Vec2::new(0., -1.);

    // let max_toi = 10.0;

    draw_line(&ray_pos, &ray_dir, max_toi, lines);

    let solid = false;
    let filter =
        QueryFilter::new().groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1).into());

    if let Some((_entity, _toi)) =
        rapier_context.cast_shape(ray_pos, shape_rot, ray_dir, &shape, max_toi, filter)
    {
        println!("Detect!");
        match *is_player_in_floor {
            IsPlayerInFloor(true) => {
                *can_jump = CanJump(true);
            }
            IsPlayerInFloor(false) => {
                *is_player_in_floor = IsPlayerInFloor(true);
                *can_jump = CanJump(true);
            }
        }
    } else {
        match *is_player_in_floor {
            IsPlayerInFloor(false) => {
                *can_jump = CanJump(false);
            }
            IsPlayerInFloor(true) => {
                println!("left floor");
                *is_player_in_floor = IsPlayerInFloor(false);
                *can_jump = CanJump(false);
            }
        }
    }
}
fn add_debug_view(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
    world: &World,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = player_q.get_single().unwrap();
    // world.query()
    commands.entity(player).insert_bundle(MaterialMesh2dBundle {
        material: materials.add(ColorMaterial {
            color: Color::rgba(1., 1., 1., 0.5),
            ..Default::default()
        }),
        mesh: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::new(PLAYER_SIZE, 5.),
                flip: false,
            }))
            .into(),

        ..default()
    });

    // commands.entity(player)
    // println!("COMPONENTS PLAYER- {:?}", playerCommands.log_components())
}

fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    is_on_wall: Res<IsOnWall>,
    mut dbg: ResMut<DebugRenderContext>,
    mut can_jump: ResMut<CanJump>,
    mut player_q: Query<(Entity, &mut Velocity), With<Player>>,
) {
    // print!("ok")
    if keys.just_pressed(KeyCode::F) {
        dbg.enabled = !dbg.enabled;
    }

    let mut player = player_q.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::W) {
        match *can_jump {
            CanJump(true) => {
                println!("can jump!");
                commands.entity(player.0).insert(ExternalImpulse {
                    impulse: vec2(0., 220.),
                    torque_impulse: 1.,
                });
                *can_jump = CanJump(false);
            }
            CanJump(false) => (),
        }
    }
    if keys.pressed(KeyCode::A) {
        match *is_on_wall {
            IsOnWall {
                is_on_wall: true,
                side: Some(WallSide::Left),
            } => {
                (*player.1).linvel = Vec2::new(-200., -200.);
            }
            _ => {
                (*player.1).linvel = vec2(-200.0, (*player.1).linvel.y);
            }
        }
    }
    if keys.pressed(KeyCode::D) {
        match *is_on_wall {
            IsOnWall {
                is_on_wall: true,
                side: Some(WallSide::Right),
            } => {
                (*player.1).linvel = Vec2::new(200., -200.);
            }
            _ => (*player.1).linvel = Vec2::new(200., (*player.1).linvel.y),
        }
        println!("LINVEL : {:?}", (*player.1).linvel)
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
        .insert(CollisionGroups::new(Group::ALL, Group::GROUP_2))
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
        .insert(CollisionGroups::new(Group::ALL, Group::GROUP_1))
        .add_child(mesh_container);

    commands
        .spawn()
        .insert(Name::from("Wall2"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(220., 0., 0.),
            ..Default::default()
        })
        .insert(Collider::cuboid(TILE_HEIGHT / 2., TILE_HEIGHT * 2.))
        .insert(Friction::default())
        .insert(CollisionGroups::new(Group::ALL, Group::GROUP_1));
    commands
        .spawn()
        .insert(Name::from("Wall1"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(-120., -120., 0.),
            ..Default::default()
        })
        .insert(Collider::cuboid(TILE_HEIGHT / 2., TILE_HEIGHT * 2.))
        .insert(Friction::default())
        .insert(CollisionGroups::new(Group::ALL, Group::GROUP_1));
}
