use crate::{
    components::{CameraBorders, CameraControlTag},
    config::{ArenaConfig, CameraConfig},
    input::AxisBinding,
    input::MovementBindingTypes,
    minions::WorldBorders,
};
use amethyst::{
    controls::WindowFocus,
    core::{
        geometry::Plane,
        math::{Point2, Vector2},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
    renderer::camera::{ActiveCamera, Camera},
    shred::ReadExpect,
    window::ScreenDimensions,
};

#[derive(SystemDesc)]
pub struct BorderSystem{
	first_run: bool,
}

impl Default for BorderSystem{
	fn default() -> Self{
		BorderSystem{
			// so that the system runs on startup
			first_run: true
		}
	}
}

// this System calculates the borderes of the camera
// they have to change every-time the user zooms in/out
impl<'s> System<'s> for BorderSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        Read<'s, WorldBorders>,
        Read<'s, ArenaConfig>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, CameraBorders>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            active_camera,
            world_borders,
            arena_config,
            screen_dimensions,
            mut camera_borders,
            input,
        ): Self::SystemData,
    ) {
        let (width, height) = { (screen_dimensions.width(), screen_dimensions.height()) };
        let zoom = input.axis_value(&AxisBinding::Zoom).unwrap_or(0.0);
        // only recalculate borders when there is zoom-change
        if zoom != 0.0 || self.first_run {
			self.first_run = false; // this runs every loop but is only needed once
            let mut camera_join = (&cameras, &transforms, &mut camera_borders).join();
            if let Some((camera, camera_transform, mut camera_border)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // bot and top borders
                let ray = camera.screen_ray(
                    Point2::new(width / 2.0, height),
                    Vector2::new(width, height),
                    &camera_transform,
                );
                let camera_translation = camera_transform.translation();
                let d = ray.intersect_plane(&Plane::with_y(0.0)).unwrap();
                let distance = camera_translation.z - ray.at_distance(d).z + arena_config.tile_size;
                camera_border.top = world_borders.top + distance;
                camera_border.bottom = world_borders.bottom + distance;

                // right and left borders
                let ray = camera.screen_ray(
                    Point2::new(0.0, height),
                    Vector2::new(width, height),
                    &camera_transform,
                );
                let d = ray.intersect_plane(&Plane::with_y(0.0)).unwrap();
                let distance = camera_translation.x - ray.at_distance(d).x;
                camera_border.left = world_borders.left + distance.abs() - arena_config.tile_size;
                camera_border.right = world_borders.right - distance.abs() + arena_config.tile_size;
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CameraControlTag>,
        Read<'s, WindowFocus>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, CameraConfig>,
        Read<'s, Time>,
        ReadStorage<'s, CameraBorders>,
    );

    fn run(
        &mut self,
        (mut transforms, camera_tag, focus, input, config, time, camera_borders): Self::SystemData,
    ) {
        let focused = focus.is_focused;
        for (transform, _, camera_borders) in (&mut transforms, &camera_tag, &camera_borders).join()
        {
			// window-focus
            if focused {
                let time_delta = time.delta_seconds();
				let zoom = input.axis_value(&AxisBinding::Zoom).unwrap_or(0.0);
				
				let z =  transform.translation().z;
				let height = transform.translation().y;
				if !( height >= 10.0 && zoom < 0.0) && !(height <= 1.0 && zoom > 0.0) 
				&& !( z >= camera_borders.top && zoom < 0.0) && !(z <= camera_borders.bottom && zoom > 0.0 ) {
                    transform.move_forward(zoom);
                }
                let translation = transform.translation_mut();
                translation.y = translation.y.clamp(1.0, 10.0);

                let right = input.axis_value(&AxisBinding::Right).unwrap_or(0.0);
                translation.x += right * config.movement_factor * time_delta;
                translation.x = translation
                    .x
                    .clamp(camera_borders.left, camera_borders.right);

                let forward = input.axis_value(&AxisBinding::Forward).unwrap_or(0.0);
                translation.z += -forward * config.movement_factor * time_delta;
                translation.z = translation
                    .z
                    .clamp(camera_borders.bottom, camera_borders.top);
            }
        }
    }
}
