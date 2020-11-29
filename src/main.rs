use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy,
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        types::DefaultBackend, RenderDebugLines, RenderShaded3D, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};
use std::time::Duration;

mod bundle;
mod components;
mod config;
mod input;
mod minions;
mod systems;
use crate::{
    bundle::MinionsBundle, config::MinionsConfig, input::MovementBindingTypes, minions::Minions,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // prepare data for GameDataBuilder
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let input_path = app_root.join("config").join("input.ron");
    let input_bundle =
        InputBundle::<MovementBindingTypes>::new().with_bindings_from_file(&input_path)?;
    let assets_dir = app_root.join("assets/");
    let minions_config = MinionsConfig::load("config/config.ron")?;

    // create game_data with GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(MinionsBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.05, 0.05, 0.05, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;
    let mut game = Application::build(assets_dir, Minions::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .with_resource(minions_config.arena)
        .with_resource(minions_config.camera)
        .build(game_data)?;
    game.run();
    Ok(())
}
