// all config-related stuff

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub depth: i16,
    pub width: i16,
    pub tile_size: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CameraConfig {
    pub camera_tilt: f32,
    pub movement_factor: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MinionsConfig {
    pub arena: ArenaConfig,
    pub camera: CameraConfig,
}
