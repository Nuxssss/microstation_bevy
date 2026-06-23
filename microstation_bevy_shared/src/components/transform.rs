use crate::components::ToBevyComponent;
use bevy::prelude::{Component, Reflect};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct Transform {
    #[serde(default)]
    parent: Option<String>,
    #[serde(default)]
    anchored: bool,
}
