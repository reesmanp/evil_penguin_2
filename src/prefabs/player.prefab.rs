use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::{Transform, math::Vector3},
    derive::PrefabData,
    ecs::{Entity},
    renderer::{Camera, camera::CameraPrefab, light::{SpotLight, Light}, palette::Srgb},
    Error
};

use serde::{Deserialize, Serialize};

use crate::{
    components::{BatteryComponent, PlayerComponent},
    util::constants::FLASHLIGHT_DEFAULT_INTENSITY
};

#[derive(Clone, Debug, Deserialize, PrefabData, Serialize)]
pub struct PlayerPrefabData {
    #[serde(default)]
    player: PlayerComponent,
    #[serde(default = "get_default_player_light_deserialization")]
    light: Light,
    battery: BatteryComponent,
    #[serde(default = "get_default_player_camera_deserialization")]
    camera: CameraPrefab,
    #[serde(default)]
    transform: Transform
}

fn get_default_player_camera_deserialization() -> CameraPrefab {
    let camera = Camera::standard_3d(1024.0, 768.0);
    let perspective = camera.projection().as_perspective().unwrap();
    CameraPrefab::Perspective {
        aspect: perspective.aspect(),
        fovy: perspective.fovy(),
        zfar: perspective.far(),
        znear: perspective.near()
    }
}

fn get_default_player_light_deserialization() -> Light {
    let flashlight = SpotLight {
        angle: 10.0,
        color: Srgb::new(1.0, 1.0, 0.8),
        intensity: FLASHLIGHT_DEFAULT_INTENSITY,
        direction: Vector3::new(0.0, 0.0, -1.0),
        range: 10.0,
        smoothness: 0.5,
        ..SpotLight::default()
    };

    Light::from(flashlight)
}
