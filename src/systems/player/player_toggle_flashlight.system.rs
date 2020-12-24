use amethyst::{
    core::Time,
    ecs::{System, Join, ReadStorage, Read, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::light::Light
};

use log::debug;

use crate::{
    components::{
        BatteryComponent,
        PlayerComponent
    },
    util::constants::{FLASHLIGHT_TOGGLE_THRESHOLD, FLASHLIGHT_BATTERY_AMOUNT, FLASHLIGHT_DEFAULT_INTENSITY}
};

#[derive(Default)]
pub struct PlayerToggleFlashlightSystem {
    time_since_last_toggle: f32
}

impl<'a> System<'a> for PlayerToggleFlashlightSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, BatteryComponent>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
        WriteStorage<'a, Light>
    );

    fn run(&mut self, (player, batteries, input, time, mut lights): Self::SystemData) {
        self.time_since_last_toggle += time.delta_seconds();

        if self.time_since_last_toggle > FLASHLIGHT_TOGGLE_THRESHOLD {
            if let Some((_player, battery, light)) = (&player, &batteries, &mut lights).join().next() {
                if let Light::Spot(flashlight) = light {
                    if battery.0 > 0 {
                        if let Some(toggle) = input.action_is_down("flashlight") {
                            if flashlight.intensity == 0.0 && toggle {
                                let battery_level = battery.0 as f32 / battery.1 as f32;
                                flashlight.intensity = FLASHLIGHT_DEFAULT_INTENSITY * battery_level;
                                debug!("Flashlight Toggled On");
                            } else if flashlight.intensity > 0.0 && toggle {
                                flashlight.intensity = 0.0;
                                debug!("Flashlight Toggled Off");
                            }

                            if toggle {
                                self.time_since_last_toggle = 0.0;
                            }
                        }
                    } else if flashlight.intensity > 0.0 {
                        debug!("Flashlight Toggled Off: Low Battery");
                        flashlight.intensity = 0.0;
                    }
                }
            }
        }
    }
}
