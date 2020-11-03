use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::TransformBundle,
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderDebugLines, RenderShaded3D, RenderSkybox, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    utils::scene::BasicScenePrefab,
    Application, GameDataBuilder,
};

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

mod minions;
use crate::minions::Minions;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
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

    let mut game = Application::new(assets_dir, Minions::default(), game_data)?;
    game.run();
    Ok(())
}
