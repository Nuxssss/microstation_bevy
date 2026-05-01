use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use crate::components::ToBevyComponent;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Transform {
    #[serde(default)]
    parent: Option<String>,
    #[serde(default)]
    anchored: bool,
}

