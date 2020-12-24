use amethyst::{
    assets::{ProgressCounter, AssetLoaderSystemData},
    ecs::{Dispatcher, DispatcherBuilder},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    ui::FontHandle,
    renderer::{
        shape::Shape,
        rendy::mesh::{Normal, Tangent, TexCoord, Position},
        MaterialDefaults, Material, Mesh
    },
    core::Transform
};

#[derive(Default)]
pub struct StartMenuState;

impl SimpleState for StartMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }
}
