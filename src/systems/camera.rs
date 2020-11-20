use crate::{
    components::CameraControlTag, config::ArenaConfig, input::AxisBinding, minions::WorldBorders,
};
use amethyst::{
    controls::WindowFocus,
    core::math::distance,
    core::{
        geometry::Plane,
        math::{Point2, Point3, Vector2},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::InputHandler,
    renderer::camera::{ActiveCamera, Camera},
    shred::ReadExpect,
    window::ScreenDimensions,
};

use crate::{components::CameraBorders, config::CameraConfig, input::MovementBindingTypes};

pub struct BorderSystem;

// this System calculates the borderes of the camera
// they have to change every-time the user zooms in/out
// at the moment it runs every tick, so it's way too often
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
        ): Self::SystemData,
    ) {
        let (width, height) = { (screen_dimensions.width(), screen_dimensions.height()) };

        let mut camera_join = (&cameras, &transforms, &mut camera_borders).join();
        if let Some((camera, camera_transform, mut camera_border)) = active_camera
            .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
        {
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

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CameraControlTag>,
        Read<'s, WindowFocus>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, CameraConfig>,
        ReadStorage<'s, CameraBorders>,
    );

    fn run(
        &mut self,
        (mut transforms, camera_tag, focus, input, config, camera_borders): Self::SystemData,
    ) {
        let focused = focus.is_focused;
        for (transform, _, camera_borders) in (&mut transforms, &camera_tag, &camera_borders).join()
        {
            if focused {
                let zoom = input.axis_value(&AxisBinding::Zoom).unwrap_or(0.0);
                transform.move_forward(zoom);

                let isometry = &mut transform.isometry_mut().translation.vector;
                isometry.y = isometry.y.clamp(1.0, 5.0);

                let right = input.axis_value(&AxisBinding::Right).unwrap_or(0.0);
                isometry.x += right * config.movement_factor;
                isometry.x = isometry.x.clamp(camera_borders.left, camera_borders.right);

                let forward = input.axis_value(&AxisBinding::Forward).unwrap_or(0.0);
                isometry.z += -forward * config.movement_factor;
                isometry.z = isometry.z.clamp(camera_borders.bottom, camera_borders.top);
            }
        }
    }
}
