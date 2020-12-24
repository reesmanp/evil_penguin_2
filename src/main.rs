#[macro_use]
extern crate const_concat;

use amethyst::{
    assets::{PrefabLoaderSystemDesc},
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::{Application, GameDataBuilder},
    renderer::{types::DefaultBackend, RenderingBundle, RenderToWindow, RenderPbr3D},
    ui::{RenderUi, UiBundle}
};

mod components;
mod prefabs;
mod resources;
mod states;
mod systems;
mod util;

use prefabs::PlayerPrefabData;
use states::menu::StartMenuState;
use states::RunState;
use util::{
    constants::{
        ASSETS_PATH,
        DISPLAY_CONFIG_PATH,
        INPUT_BINDINGS_PATH
    }
};

fn main() -> amethyst::Result<()> {
    start_logging();

    // Create Bundles
    let input_bundle = initialize_input_bundle();
    let transform_bundle = initialize_transform_bundle();
    let rendering_bundle = initialize_rendering_bundle();
    let ui_bundle = initialize_ui_bundle();

    // Create Game Data
    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<PlayerPrefabData>::default(), "", &[])
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(rendering_bundle)?
        .with_bundle(ui_bundle)?;

    // Create Starting State
    // let start_menu_state = StartMenuState::default();
    // TODO: Put start menu state back or make a manager state
    let run_state = RunState::default();

    // Create Game
    let mut game = Application::new(ASSETS_PATH, run_state, game_data)?;
    game.run();

    Ok(())
}

/// Initializes Amethyst Logger
/// Depends on type of compilation
///
/// Debug compilation --> Debug log level
/// Release compilation --> Info log level
fn start_logging() {
    let logger_level;
    if cfg!(debug_assertions) {
        logger_level = amethyst::LogLevelFilter::Debug;
    } else {
        logger_level = amethyst::LogLevelFilter::Info;
    }

    let logger_config = amethyst::LoggerConfig {
        level_filter: logger_level,
        ..amethyst::LoggerConfig::default()
    };

    amethyst::start_logger(logger_config);
}

/// Initialize InputBundle
///
/// Takes in INPUT_BINDINGS file as StringBindings for input controls
fn initialize_input_bundle() -> InputBundle<StringBindings> {
    InputBundle::<StringBindings>::new().with_bindings_from_file(INPUT_BINDINGS_PATH).unwrap()
}

/// Initializes TransformBundle
fn initialize_transform_bundle() -> TransformBundle<'static> {
    TransformBundle::new()
}

/// Initializes RenderingBundle
///
/// Uses DISPLAY_CONFIG file
/// Background drawn with 0, 0, 0 RGB
/// Uses DefaultBackend
/// Uses physically based rendering RenderPbr3D
fn initialize_rendering_bundle() -> RenderingBundle<DefaultBackend> {
    RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(DISPLAY_CONFIG_PATH)
                         .unwrap()
                         .with_clear([0.001, 0.001, 0.001, 1.0]),
        )
        .with_plugin(RenderPbr3D::default())
        .with_plugin(RenderUi::default())
}

/// Initializes UiBundle
fn initialize_ui_bundle() -> UiBundle<StringBindings> {
    UiBundle::<StringBindings>::new()
}
