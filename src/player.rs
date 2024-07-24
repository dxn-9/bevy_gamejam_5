use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::GravityScale,
};

use crate::{
    components::{Player, PlayerMesh},
    constants::{GRAVITY, GROUND_TIMER, PLAYER_VELOCITY},
    resources::MovementInput,
};

pub fn player_movement(
    time: Res<Time>,
    mut input: ResMut<MovementInput>,
    mut player: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
    mut q_player_mesh: Query<&mut Transform, (With<PlayerMesh>, Without<Player>)>,
    mut vertical_movement: Local<f32>,
) {
    let Ok((transform, mut controller, output)) = player.get_single_mut() else {
        return;
    };
    let Ok(mut mesh_transform) = q_player_mesh.get_single_mut() else {
        println!("no mesh :(");
        return;
    };
    let mut movement = Vec3::new(input.x, 0.0, input.z) * PLAYER_VELOCITY;
    let delta_time = time.delta_seconds();

    if output.map(|o| o.grounded).unwrap_or(false) {
        *vertical_movement = 0.0;
    }
    // If we are grounded we can jump
    movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    controller.translation = Some(transform.rotation * (movement * delta_time));

    // Update player mesh direction
    let t = mesh_transform.translation.clone();
    mesh_transform.look_at(t + vec3(input.x, 0.0, input.z), Vec3::Y);

    {
        // Clear input
        **input = Vec3::ZERO;
    }
}
