use amethyst::{
	assets::Handle, 
	core::{
		math::{Point2, Vector2},
		Transform,
		geometry::Plane
	}, 
	ecs::{Entity, Entities, Join, ReadStorage, System, SystemData, Read, WriteStorage, ReadExpect}, 
	input::InputHandler, 
	derive::SystemDesc,
	renderer::{ActiveCamera, Camera, Material}, 
	window::ScreenDimensions
};

use std::default::Default;


#[derive(Default, SystemDesc)]
#[system_desc(name(HoverSystemDesc))]
pub struct HoverSystem{
	current_hover: Option<Entity>
}

impl Default for HoverSystemDesc{
	fn default() -> Self {
		HoverSystemDesc{
			current_hover: None
		}
	}
}

use crate::{
    components::PieceInfo, config::ArenaConfig, input::MovementBindingTypes, minions::WorldBorders,
};

// this system lightens the color of hovered planes
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
		WriteStorage<'s, Handle<Material>>,
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
			mut material_handles,
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
					// get coords of plane
                    let index_x =
                        ((position.x + world_borders.left.abs()) / arena_config.tile_size).floor();
                    let index_z = ((position.z + world_borders.bottom.abs())
                        / arena_config.tile_size)
						.floor();
						
					// change plane-albedo to a lighter gray
                    for (entity, piece_info) in (&entities, &piece_infos).join() {
                        if piece_info.x == index_x as i16 && piece_info.z == index_z as i16 {
                            if self.current_hover != Some(entity) {

								if let Some(entity2) = self.current_hover {
									material_handles.remove(entity2);
                                	material_handles.insert(entity2, piece_info.basic_material.clone()).unwrap();
								}

								self.current_hover = Some(entity);
                                material_handles.remove(entity);
                                material_handles.insert(entity, piece_info.hover_material.clone()).unwrap();
							}
							break
						} 
					}
				}
				else {
					if let Some(entity2) = self.current_hover.take() {
						let piece_info = piece_infos.get(entity2).expect("Why is there no plane-info to this plane?");
						material_handles.remove(entity2);
						material_handles.insert(entity2, piece_info.basic_material.clone()).unwrap();
					}
				}
            }
        }
    }
}
