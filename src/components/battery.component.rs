use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, Entity, HashMapStorage, WriteStorage},
    Error
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Component, Debug, Deserialize, PrefabData, Serialize)]
#[storage(HashMapStorage)]
#[prefab(Component)]
pub struct BatteryComponent(pub usize, pub usize);
