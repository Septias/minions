use crate::systems::{BorderSystem, CameraSystem, HoverSystem};
use amethyst::{
    controls::MouseFocusUpdateSystemDesc,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
    prelude::SystemDesc,
};
pub struct MinionsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MinionsBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(BorderSystem, "camera_border", &[]);
        builder.add(
            CameraSystem,
            "camera_system",
            &["input_system", "camera_border"],
        );
        /* builder.add(
            MouseFocusUpdateSystemDesc::default().build(world),
            "mouse_focus",
            &["camera_system"],
        ); */
        builder.add(HoverSystem, "hover_system", &[]);
        Ok(())
    }
}
