use crate::systems::{BorderSystem, CameraSystem, HoverSystemDesc};
use amethyst::{
    controls::MouseFocusUpdateSystemDesc,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
    prelude::SystemDesc,
};
use std::default::Default;
pub struct MinionsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MinionsBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(BorderSystem::default(), "camera_border", &[]);
        builder.add(
            CameraSystem,
            "camera_system",
            &["input_system", "camera_border"],
        );
        builder.add(
            MouseFocusUpdateSystemDesc::default().build(world),
            "mouse_focus",
            &["camera_system"],
        );
        builder.add(HoverSystemDesc::default().build(world), "hover_system", &[]);
        Ok(())
    }
}
