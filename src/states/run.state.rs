use amethyst::{
    assets::{AssetLoaderSystemData, PrefabLoader, RonFormat},
    core::{ArcThreadPool, Transform, math::Vector3},
    ecs::{Dispatcher, DispatcherBuilder},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::{
        camera::Camera,
        light::{Light, SpotLight},
        MaterialDefaults, Material, Mesh,
        palette::Srgb,
        rendy::mesh::{Normal, Tangent, TexCoord, Position},
        shape::Shape,
    },
    winit::{Event, WindowEvent, DeviceEvent}
};

use log::info;

use crate::{
    components::{
        BatteryComponent
    },
    prefabs::PlayerPrefabData,
    resources::{MouseMovementResource, WindowFocusedResource},
    systems::player::{
        PlayerBatteryDrainSystem,
        PlayerLookSystem,
        PlayerMovementSystem,
        PlayerToggleFlashlightSystem,
        PlayerToggleRunSystem
    },
    util::constants::PLAYER_PREFAB
};

#[derive(Default)]
pub struct RunState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>
}

impl<'a, 'b> SimpleState for RunState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        info!("Initializing: Run State");

        self.initialize_dispatcher(world);
        self.create_player_entity(world);
        self.create_test_sphere(world);
        self.create_ground(world);
        self.create_resources(world);

        info!("Done: Run State");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            // Quit Scenario
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            // Check to see if the window is focused
            } else if let Event::WindowEvent { ref event, .. } = *e {
                if let WindowEvent::Focused(is_focused) = event {
                    data.world.write_resource::<WindowFocusedResource>().0 = *is_focused;
                }
            // Check to see if there is mouse motion
            } else if let Event::DeviceEvent { ref event, .. } = *e {
                if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
                    let mut mouse_resource = data.world.write_resource::<MouseMovementResource>();
                    mouse_resource.0 = *x as f32;
                    mouse_resource.1 = *y as f32;
                }
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

impl<'a, 'b> RunState<'a, 'b> {
    fn initialize_dispatcher(&mut self, world: &mut World) {
        info!("Initializing: Run State Dispatcher");

        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(PlayerToggleRunSystem, "player_toggle_run_system", &[]);
        dispatcher_builder.add(PlayerMovementSystem,"player_movement_system",&["player_toggle_run_system"]);
        dispatcher_builder.add(PlayerBatteryDrainSystem::default(), "player_battery_drain_system", &[]);
        dispatcher_builder.add(PlayerToggleFlashlightSystem::default(), "player_toggle_flashlight_system",&["player_battery_drain_system"]);
        dispatcher_builder.add(PlayerLookSystem, "player_look_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        info!("Done: Run State Dispatcher");
    }

    fn create_player_entity(&self, world: &mut World) {
        info!("Creating: Player Entity");

        let prefab_handle = world
            .exec(|loader: PrefabLoader<'_, PlayerPrefabData>| {
                loader.load(
                    PLAYER_PREFAB,
                    RonFormat,
                    (),
                )
            });

        world
            .create_entity()
            .with(prefab_handle)
            .build();

        info!("Done: Player Entity")
    }

    fn create_test_sphere(&self, world: &mut World) {
        info!("Creating: Test Sphere Entity");

        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Sphere(100, 100)
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                    .into(),
                (),
            )
        });
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
            loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        });

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, -7.0);
        world.create_entity().with(mesh).with(material).with(transform).build();

        info!("Done: Test Sphere Entity");
    }

    // TODO: find out why this is not rendering
    fn create_ground(&self, world: &mut World) {
        info!("Creating: Ground");

        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Plane(Some((100, 100)))
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                    .into(),
                (),
            )
        });
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
            loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        });

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, -10.0, 0.0);
        world.create_entity().with(mesh).with(material).with(transform).build();

        info!("Done: Ground");
    }

    fn create_resources(&mut self, world: &mut World) {
        info!("Inserting: Resources");

        world.insert(MouseMovementResource::default());
        world.insert(WindowFocusedResource::default());

        info!("Done: Resources");
    }
}
