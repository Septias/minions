use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub height: i32,
    pub width: i32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 5,
            width: 5,
        }
    }
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MinionsConfig {
    pub arena: ArenaConfig,
}
