use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, Entity, HashMapStorage, WriteStorage},
    Error
};

#[derive(Clone, Copy, Component, Debug, Default, PrefabData)]
#[storage(HashMapStorage)]
#[prefab(Component)]
pub struct RunComponent;
