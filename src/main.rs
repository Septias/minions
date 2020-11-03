use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
    core::math::Point3,
    core::{
        ecs::{Builder, WorldExt},
        math::Vector3,
        TransformBundle,
    },
    renderer::debug_drawing::DebugLines,
    renderer::debug_drawing::DebugLinesComponent,
    renderer::debug_drawing::DebugLinesParams,
    renderer::palette::Srgba,
    renderer::RenderSkybox,
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderDebugLines, RenderShaded3D, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    utils::scene::BasicScenePrefab,
    Application, GameData, GameDataBuilder, SimpleState, StateData,
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;
struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sphere.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();

        data.world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        data.world.insert(DebugLinesParams { line_width: 1.0 });

        // Setup debug lines as a component and add lines to render axes & grid
        let mut debug_lines_component = DebugLinesComponent::with_capacity(100);

        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0001, 0.0),
            Vector3::new(0.2, 0.0, 0.0),
            Srgba::new(1.0, 0.0, 0.23, 1.0),
        );

        // Y-axis (yellowish-green)
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.2, 0.0),
            Srgba::new(0.5, 0.85, 0.1, 1.0),
        );

        // Z-axis (blue)
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0001, 0.0),
            Vector3::new(0.0, 0.0, 0.2),
            Srgba::new(0.2, 0.75, 0.93, 1.0),
        );

        data.world.register::<DebugLinesComponent>();
        data.world
            .create_entity()
            .with(debug_lines_component)
            .build();
    }
}

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

    let mut game = Application::new(assets_dir, Example, game_data)?;
    game.run();
    Ok(())
}
