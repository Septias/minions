// Initialize game world

use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::{
        ecs::{Builder, WorldExt},
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::palette::Srgb,
    renderer::Camera,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        light::{DirectionalLight, Light},
        loaders::load_from_linear_rgba,
        palette::{LinSrgba, Srgba},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Material, MaterialDefaults, Mesh, Texture,
    },
    SimpleState,
};

use crate::{components::CameraControlTag, config::ArenaConfig};
#[derive(Default)]
pub struct Minions {}

impl SimpleState for Minions {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let (width, depth, tile_size, camera_tilt) = {
            let arena_config = data.world.read_resource::<ArenaConfig>();
            (
                arena_config.width,
                arena_config.depth,
                arena_config.tile_size,
                arena_config.camera_tilt,
            )
        };
        initialize_debug_lines(data.world);
        initialize_ground(data.world, width, depth, tile_size);
        initialize_camera(data.world, camera_tilt);
        initialize_light(data.world);
    }
}

fn initialize_debug_lines(world: &mut World) {
    world.insert(DebugLines::new());
    world.insert(DebugLinesParams { line_width: 1.0 });
    let mut debug_lines_component = DebugLinesComponent::with_capacity(3);

    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0001, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Srgba::new(1.0, 0.0, 0.23, 1.0),
    );
    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Srgba::new(0.5, 0.85, 0.1, 1.0),
    );
    debug_lines_component.add_direction(
        Point3::new(0.0, 0.0001, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        Srgba::new(0.2, 0.75, 0.93, 1.0),
    );

    world.register::<DebugLinesComponent>();
    world.create_entity().with(debug_lines_component).build();
}

fn initialize_ground(world: &mut World, width: i32, depth: i32, tile_size: f32) {
    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let mesh = create_plane(world);
    let albedo = create_albedo(world);
    let roughness = 1.0f32;
    let metallic = 1.0f32;

    // create material
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

    // initialize planes
    let x0 = -(tile_size * width as f32 / 2.);
    let z0 = -(tile_size * depth as f32 / 2.);

    for x in 0..width {
        for y in 0..depth {
            let mut pos = Transform::default();
            pos.append_rotation_x_axis(-1.5707);
            pos.set_translation_xyz(
                x0 + (tile_size * x as f32),
                0.0,
                z0 + (tile_size * y as f32),
            );
            pos.set_scale(Vector3::new(
                0.5 * tile_size,
                0.5 * tile_size,
                0.5 * tile_size,
            ));
            world
                .create_entity()
                .with(pos)
                .with(mesh.clone())
                .with(mtl.clone())
                .build();
        }
    }

    // create grid-lines
    let mut debug_lines_component = DebugLinesComponent::with_capacity((width * depth) as usize);
    let main_color = Srgba::new(0.0, 0.0, 0.0, 0.8);
    for x in 0..=width {
        let position = Point3::new(
            x0 - tile_size / 2.0 + (x as f32 * tile_size),
            0.0,
            z0 - tile_size / 2.00,
        );
        let direction = Vector3::new(0.0, 0.0, tile_size * depth as f32);
        debug_lines_component.add_direction(position, direction, main_color);
    }
    for z in 0..=depth {
        let position = Point3::new(
            x0 - tile_size / 2.0,
            0.0,
            z0 - tile_size / 2.00 + (z as f32 * tile_size),
        );
        let direction = Vector3::new(tile_size * width as f32, 0.0, 0.0 as f32);
        debug_lines_component.add_direction(position, direction, main_color);
    }
    world.create_entity().with(debug_lines_component).build();
}

fn create_plane(world: &mut World) -> Handle<Mesh> {
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
            load_from_linear_rgba(LinSrgba::new(0.005, 0.005, 0.005, 1.0)).into(),
            (),
        )
    })
}

fn initialize_camera(world: &mut World, camera_tilt: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 2.0, 5.0);
    transform.prepend_rotation_x_axis(camera_tilt);
    world
        .create_entity()
        .with(Camera::perspective(1.3, 1.0471975512, 0.1))
        .with(transform)
        .with(CameraControlTag)
        .build();
}

fn initialize_light(world: &mut World) {
    let mut pos = Transform::default();
    pos.prepend_translation_y(10.0);

    let light: Light = DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        intensity: 5.0,
        direction: Vector3::new(0.0, -1.0, 0.0),
    }
    .into();
    world.create_entity().with(light).with(pos).build();
}
