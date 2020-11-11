use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, Entity, NullStorage, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CameraControlTag;

impl Component for CameraControlTag {
    type Storage = NullStorage<CameraControlTag>;
}
