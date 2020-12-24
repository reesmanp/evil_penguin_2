use amethyst::{
    core::Transform,
    ecs::{System, Join, Read, ReadStorage, WriteStorage, ReadExpect},
    renderer::light::Light,
    winit::Window
};

use crate::{
    components::PlayerComponent,
    resources::{MouseMovementResource, WindowFocusedResource}
};

pub struct PlayerLookSystem;

impl<'a> System<'a> for PlayerLookSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        Read<'a, MouseMovementResource>,
        Read<'a, WindowFocusedResource>,
        ReadExpect<'a, Window>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Light>
    );

    fn run(&mut self, (player, mouse, focused, window, mut transform, mut lights): Self::SystemData) {
        if focused.0 {
            window.grab_cursor(true).unwrap();
            window.hide_cursor(true);

            if let Some((_player, player_transform, light)) = (&player, &mut transform, &mut lights).join().next() {
                let look_x = -mouse.0.to_radians();
                let look_y = -mouse.1.to_radians();

                if look_x != 0.0 && look_y != 0.0 {
                    player_transform.append_rotation_x_axis(look_y);
                    player_transform.append_rotation_y_axis(look_x);

                    if let Light::Spot(flashlight) = light {
                        let current_rotation_matrix = player_transform.rotation().to_rotation_matrix();
                        flashlight.direction = current_rotation_matrix.transform_vector(&flashlight.direction);
                    }
                }
            }
        }
    }
}
