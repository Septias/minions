use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy,
    core::TransformBundle,
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderDebugLines, RenderShaded3D, RenderSkybox, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};
use std::{path::Path, time::Duration};

mod config;
mod minions;
use crate::config::ArenaConfig;
use crate::minions::Minions;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");
    let arena_config = ArenaConfig::load("config/config.ron")?;
    println!("{:?}", &arena_config);
    let game_data = GameDataBuilder::default()
        // .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;
    let mut game = Application::build(assets_dir, Minions::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .with_resource(arena_config)
        .build(game_data)?;
    game.run();
    Ok(())
}
