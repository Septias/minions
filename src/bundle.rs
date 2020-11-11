use crate::systems::CameraSystem;
use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};

pub struct MinionsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MinionsBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CameraSystem, "camera_system", &["input_system"]);
        Ok(())
    }
}
