use amethyst::{
    assets::{PrefabLoader, RonFormat},
    core::{
        ecs::{Builder, WorldExt},
        math::{Point3, Vector3},
    },
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
    renderer::palette::Srgba,
    SimpleState,
};

use crate::MyPrefabData;

#[derive(Default)]
pub struct Minions {}

impl SimpleState for Minions {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sphere.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();
        initialize_debug_lines(data.world);
    }
}

fn initialize_debug_lines(world: &mut World) {
    world.insert(DebugLines::new());
    world.insert(DebugLinesParams { line_width: 1.0 });
    let mut debug_lines_component = DebugLinesComponent::with_capacity(3);

    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0001, 0.0),
        Vector3::new(0.2, 0.0, 0.0),
        Srgba::new(1.0, 0.0, 0.23, 1.0),
    );
    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.2, 0.0),
        Srgba::new(0.5, 0.85, 0.1, 1.0),
    );
    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0001, 0.0),
        Vector3::new(0.0, 0.0, 0.2),
        Srgba::new(0.2, 0.75, 0.93, 1.0),
    );

    world.register::<DebugLinesComponent>();
    world.create_entity().with(debug_lines_component).build();
}
