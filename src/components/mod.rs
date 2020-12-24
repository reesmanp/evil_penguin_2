#[path = "battery.component.rs"]
mod battery_component;
#[path = "player.component.rs"]
mod player_component;
#[path = "run.component.rs"]
mod run_component;

pub use self::{
    battery_component::BatteryComponent,
    player_component::PlayerComponent,
    run_component::RunComponent
};
