use amethyst::{
    core::Time,
    ecs::{System, Join, WriteStorage, Read, ReadStorage},
    renderer::light::Light
};

use log::debug;

use crate::{
    components::{BatteryComponent, PlayerComponent},
    util::constants::{
        FLASHLIGHT_BATTERY_AMOUNT,
        FLASHLIGHT_DEFAULT_INTENSITY,
        FLASHLIGHT_BATTERY_DRAIN_SPEED
    }
};

#[derive(Default)]
pub struct PlayerBatteryDrainSystem {
    time_since_last_battery_tick: f32
}

impl<'a> System<'a> for PlayerBatteryDrainSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        Read<'a, Time>,
        WriteStorage<'a, Light>,
        WriteStorage<'a, BatteryComponent>
    );

    fn run(&mut self, (player, time, mut lights, mut batteries): Self::SystemData) {
        if let Some((_player, light, battery)) = (&player, &mut lights, &mut batteries).join().next() {
            if let Light::Spot(flashlight) = light {
                if flashlight.intensity > 0.0 {
                    self.time_since_last_battery_tick += time.delta_seconds();

                    // Let's us control how fast the battery drains in seconds
                    if self.time_since_last_battery_tick > FLASHLIGHT_BATTERY_DRAIN_SPEED {
                        // Resets the counter
                        self.time_since_last_battery_tick -= FLASHLIGHT_BATTERY_DRAIN_SPEED;
                        battery.0 -= 1;

                        debug!("Flashlight Battery Level: {}", battery.0);

                        let battery_level = battery.0 as f32 / battery.1 as f32;
                        flashlight.intensity = FLASHLIGHT_DEFAULT_INTENSITY * battery_level;
                    }
                }
            }
        }
    }
}
