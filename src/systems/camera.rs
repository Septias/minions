use crate::{components::CameraControlTag, input::AxisBinding};
use amethyst::{
    controls::WindowFocus,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
};

use crate::{config::CameraConfig, input::MovementBindingTypes};

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CameraControlTag>,
        Read<'s, WindowFocus>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, CameraConfig>,
    );

    fn run(&mut self, (mut transforms, camera_tag, focus, input, config): Self::SystemData) {
        let focused = focus.is_focused;
        for (transform, _) in (&mut transforms, &camera_tag).join() {
            if focused {
                let right = input.axis_value(&AxisBinding::Right).unwrap_or(0.0);
                transform.move_right(right * config.movement_factor);

                let zoom = input.axis_value(&AxisBinding::Zoom).unwrap_or(0.0);
                transform.move_forward(zoom);

                let forward = input.axis_value(&AxisBinding::Forward).unwrap_or(0.0);
                transform.prepend_translation_z(-forward * config.movement_factor);
            }
        }
    }
}
