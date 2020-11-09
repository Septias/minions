// all config-related stuff

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub depth: i32,
    pub width: i32,
    pub tile_size: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            depth: 5,
            width: 5,
            tile_size: 1.0,
        }
    }
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MinionsConfig {
    pub arena: ArenaConfig,
}