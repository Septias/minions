use amethyst::{
    assets::{AssetLoaderSystemData, Handle, PrefabLoader, RonFormat},
    core::{
        ecs::{Builder, WorldExt},
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        loaders::load_from_linear_rgba,
        palette::{LinSrgba, Srgba},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Material, MaterialDefaults, Mesh, Texture,
    },
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
        initialize_ground(data.world);
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

fn initialize_ground(world: &mut World) {
    let (width, height) = (5, 5);

    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let mesh = create_mesh(world);
    let albedo = create_albedo(world);
    let roughness = 1.0f32;
    let metallic = 1.0f32;

    let mtl = world.exec(
        |(mtl_loader, tex_loader): (
            AssetLoaderSystemData<'_, Material>,
            AssetLoaderSystemData<'_, Texture>,
        )| {
            let metallic_roughness = tex_loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
                (),
            );

            mtl_loader.load_from_data(
                Material {
                    albedo: albedo.clone(),
                    metallic_roughness,
                    ..mat_defaults.clone()
                },
                (),
            )
        },
    );

    let mut pos = Transform::default();
    pos.append_rotation_x_axis(-1.5707);
    world.create_entity().with(mesh).with(mtl).with(pos).build();
    /* for _ in 0..width {
        for _ in 0..height {
            world.create_entity().build();
        }
    } */
}

fn create_mesh(world: &mut World) -> Handle<Mesh> {
    world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    })
}

fn create_albedo(world: &mut World) -> Handle<Texture> {
    world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
            (),
        )
    })
}
