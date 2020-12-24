use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, Entity, HashMapStorage, WriteStorage},
    Error
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Default, Debug, Deserialize, PrefabData, Serialize)]
#[storage(HashMapStorage)]
#[prefab(Component)]
pub struct PlayerComponent;
