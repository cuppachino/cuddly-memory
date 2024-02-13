use bevy::{ core_pipeline::clear_color::ClearColorConfig, prelude::*, transform::TransformSystem };
use bevy_boiler::{ average, SpawnDemoLights };
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::{ plugin::InputManagerSystem, prelude::* };

mod input;
use input::*;

fn main() {
    let mut app = App::new();

    // Default plugins
    app.add_plugins((
        DefaultPlugins.set(bevy::render::RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(
                bevy::render::settings::WgpuSettings {
                    // ! WARN this is a native only feature. It will not work with webgl or webgpu
                    features: bevy::render::settings::WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }
            ),
        }).set(WindowPlugin {
            primary_window: Some(Window {
                // // present_mode: bevy::window::PresentMode::Mailbox,
                ..Default::default()
            }),
            ..Default::default()
        }),
        bevy::pbr::wireframe::WireframePlugin,
    ))
        .insert_resource(bevy::pbr::wireframe::WireframeConfig {
            global: false,
            default_color: Color::WHITE,
        })
        .insert_resource(ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0)));

    // Game plugins
    app.add_plugins((
        InputManagerPlugin::<PlayerAction>::default(),
        (PhysicsPlugins::default(), PhysicsDebugPlugin::default()),
    ))
        .register_type::<ActionState<PlayerAction>>()
        .register_type::<LinearSpeed>()
        .register_type::<AveragePlayerPosition>()
        .init_resource::<AveragePlayerPosition>()
        .add_systems(Startup, setup)
        .add_systems(Update, move_players)
        .add_systems(
            PostUpdate,
            (calculate_average_player_position, track_average_player_position)
                .chain()
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate)
        );

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Spawn background image
    let image_handle = asset_server.load("unfinished_crown.png");
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::from(shape::Quad {
                size: Vec2::splat(10.0),
                flip: false,
            })
        ),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            base_color: Color::WHITE,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        ..Default::default()
    });

    // Spawn demo lights
    commands.spawn_demo_lights();

    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 7.0, 20.0)).looking_at(
            Vec3::ZERO,
            Vec3::Y
        ),
        ..default()
    });

    // Spawn player
    commands.spawn((
        PlayerControllerBundle::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            ..default()
        },
        // XPBD
        (
            RigidBody::Dynamic,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED.lock_translation_z(),
            ExternalImpulse::default(),
            Mass(0.125),
            Collider::cuboid(1.0, 1.0, 1.0),
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            LinearVelocity::default(),
            LinearDamping(1.0),
        ),
        // Extended
        LinearSpeed(10.0),
    ));
}

/// A config component for an entity that moves.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct LinearSpeed(pub f32);

/// Applies external impulses to players.
pub fn move_players(
    mut query: Query<(&PlayerController, &LinearSpeed, &mut ExternalImpulse)>,
    time: Res<Time>
) {
    let delta_time = time.delta_seconds();
    query.for_each_mut(|(actions, linear_speed, mut impulse)| {
        if actions.pressed(PlayerAction::Move) {
            println!("actions: {:?}", actions);
            let Some(dual_axis) = actions.clamped_axis_pair(PlayerAction::Move) else {
                return;
            };
            let amount = dual_axis.xy() * linear_speed.0 * delta_time;
            impulse.apply_impulse(amount.extend(0.0));
        }
    });
}

/// Holds the average position of all players.
#[derive(Resource, Debug, Default, Deref, DerefMut, Reflect)]
pub struct AveragePlayerPosition(pub Vec3);

/// Calculates the average position of all players.
pub fn calculate_average_player_position(
    mut average_position: ResMut<AveragePlayerPosition>,
    query: Query<&Transform, With<PlayerController>>
) {
    let positions = query.iter().map(|transform| transform.translation);
    average_position.0 = average(positions);
}

/// Smoothly transitions the camera to the average position of all players.
pub fn track_average_player_position(
    mut query: Query<&mut Transform, With<Camera3d>>,
    average_position: Res<AveragePlayerPosition>,
    time: Res<Time>
) {
    let lerp_speed = 0.5 * time.delta_seconds();
    query.for_each_mut(|mut transform| {
        let Vec2 { x, y } = transform.translation.xy().lerp(average_position.0.xy(), lerp_speed);

        *transform = Transform::from_xyz(x, y, transform.translation.z).looking_at(
            average_position.0,
            Vec3::Y
        );
    });
}
