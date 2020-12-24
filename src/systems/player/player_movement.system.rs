use amethyst::{
    core::{Time, Transform, math::Vector3},
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    input::{InputHandler, StringBindings}
};

use crate::{
    components::{PlayerComponent, RunComponent},
    util::constants::{PLAYER_RUN, PLAYER_WALK}
};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, RunComponent>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, InputHandler<StringBindings>>
    );

    fn run(&mut self, (player, run, mut transform, time, input): Self::SystemData) {
        let velocity_multiplier = match (&player, &run).join().next() {
            Some((_player, _run)) => PLAYER_RUN,
            None => PLAYER_WALK
        };

        if let Some((_player, player_transform)) = (&player, &mut transform).join().next() {
            let current_rotation_matrix = player_transform.rotation().to_rotation_matrix();

            let x_velocity = input.axis_value("move-x").unwrap_or(0.0);
            let z_velocity = input.axis_value("move-z").unwrap_or(0.0);

            let mini_velocity = Vector3::new(x_velocity, 0.0, z_velocity);
            let velocity = mini_velocity * velocity_multiplier;
            let rotated_velocity = current_rotation_matrix.transform_vector(&velocity);
            let player_translation = player_transform.translation().clone();
            let player_new_translation = player_translation + rotated_velocity * time.delta_seconds();
            player_transform.prepend_translation(player_new_translation - player_translation);
        }
    }
}
