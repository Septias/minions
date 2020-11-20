use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, Entity, NullStorage, VecStorage, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

use crate::minions::WorldBorders;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CameraControlTag;

impl Component for CameraControlTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Debug)]
pub struct CameraBorders {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Component for CameraBorders {
    type Storage = VecStorage<Self>;
}
