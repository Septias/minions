use amethyst::{
    assets::{AssetLoaderSystemData, AssetStorage},
    core::geometry::Plane,
    core::{
        math::{Point2, Vector2},
        Transform,
    },
    ecs::{Entities, Join, ReadStorage},
    input::InputHandler,
    renderer::{ActiveCamera, Camera, Material, Texture},
    shred::Write,
    shred::{Read, ReadExpect, System},
    window::ScreenDimensions,
};

pub struct HoverSystem;
use crate::{
    components::PieceInfo, config::ArenaConfig, input::MovementBindingTypes, minions::WorldBorders,
};

impl<'s> System<'s> for HoverSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, PieceInfo>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ArenaConfig>,
        Read<'s, WorldBorders>,
        Read<'s, AssetStorage<Material>>,
		Write<'s, AssetStorage<Texture>>,
		AssetLoaderSystemData<'_, Texture>
    );

    fn run(
        &mut self,
        (
            entities,
            piece_infos,
            cameras,
            transforms,
            active_camera,
            input,
            screen_dimensions,
            arena_config,
            world_borders,
			material_assets,
			loader
        ): Self::SystemData,
    ) {
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_y(0.0)).unwrap();
                let position = ray.at_distance(distance);

                // if mouse-coords in world-bords
                if world_borders.left < position.x
                    && position.x < world_borders.right
                    && world_borders.bottom < position.z
                    && position.z < world_borders.top
                {
                    let index_x =
                        ((position.x + world_borders.left.abs()) / arena_config.tile_size).floor();
                    let index_z = ((position.z + world_borders.bottom.abs())
                        / arena_config.tile_size)
                        .floor();
                    for (entity, piece_info) in (&entities, &piece_infos).join() {
                        if piece_info.x == index_x as i16 && piece_info.z == index_z as i16 {
                            let material = material_assets.get(&piece_info.material).unwrap();
							
                        }
                    }
                };
            }
        }
    }
}
