use crate::{components::CameraControlTag, input::AxisBinding};
use amethyst::{
    controls::WindowFocus,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
};
use log::info;

use crate::input::MovementBindingTypes;

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CameraControlTag>,
        Read<'s, WindowFocus>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, camera_tag, _focus, input): Self::SystemData) {
        for (transform, _) in (&mut transforms, &camera_tag).join() {
            //if focus.is_focused;
            let horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
            if horizontal.abs() > 0.0 {
                transform.move_right(horizontal * 0.01);
            }
        }
    }
}
