// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::f32::consts::PI;

use bevy::asset::AssetMetaCheck;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use calendar::update_calendar;
use components::{FreeCamera, Player, PlayerCamera};
use constants::{
    CAMERA_DISTANCE_X, CAMERA_DISTANCE_Y, CAMERA_DISTANCE_Z, CAMERA_PAN_VELOCITY,
    FREE_CAMERA_VELOCITY, GRAVITY, PLAYER_VELOCITY, STEERING_FACTOR,
};
use pan_camera::{pan_orbit_camera, PanOrbitCameraBundle, PanOrbitSettings, PanOrbitState};
use player::player_movement;
use resources::{Calendar, MovementInput, ResourcesPlugin};
use ui::{setup_ui, update_ui};
use world::{check_seasons_for_world, setup_world};

mod calendar;
mod components;
mod constants;
mod pan_camera;
mod player;
mod resources;
mod ui;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            ResourcesPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .init_resource::<Calendar>()
        .init_resource::<AmbientLight>()
        .init_resource::<MovementInput>()
        .add_systems(Startup, (setup, setup_ui, setup_world))
        .add_systems(PreUpdate, handle_input.after(InputSystem))
        .add_systems(
            Update,
            (|mut exit: EventWriter<AppExit>| {
                exit.send(AppExit::Success);
            })
            .run_if(input_just_pressed(bevy::input::keyboard::KeyCode::Escape)),
        )
        .add_systems(Update, (update_ui, update_calendar))
        .add_systems(
            FixedUpdate,
            (
                player_movement,
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
                check_seasons_for_world,
            ),
        )
        .run();
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    mut q_player_camera: Query<&mut Camera, (With<PlayerCamera>, Without<FreeCamera>)>,
    mut q_free_camera: Query<&mut Camera, (With<FreeCamera>, Without<PlayerCamera>)>,
    mut q_player: Query<
        (
            &mut KinematicCharacterController,
            &mut Velocity,
            &mut Transform,
        ),
        (With<Player>, Without<PlayerCamera>),
    >,
    mut gizmos: Gizmos,
) {
    let mut player_camera = q_player_camera.single_mut();
    let mut free_camera = q_free_camera.single_mut();
    let (mut kinematic_controller, mut velocity, player) = q_player.single_mut();

    /* Keyboard input */

    if keys.pressed(KeyCode::KeyD) {
        movement.x = 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x = -1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        movement.z = -1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.z = 1.0;
    }
    **movement = movement.normalize_or_zero();

    // Toggle between free and player camera
    if keys.just_pressed(KeyCode::KeyP) {
        if player_camera.is_active {
            player_camera.is_active = false;
            free_camera.is_active = true;
        } else {
            player_camera.is_active = true;
            free_camera.is_active = false;
        }
    }

    /* Move player according to now velocity */
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 2. - 0.1)),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Directional Light"),
    ));

    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(1.0, 0.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = -40.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();

    commands.spawn((
        camera,
        Name::new("Free Camera"),
        FreeCamera,
        BloomSettings::NATURAL,
    ));
}
