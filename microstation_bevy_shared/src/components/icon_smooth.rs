use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Serialize, Component, Default, Clone, Debug)]
pub struct IconSmooth {
    #[serde_inline_default(true)]
    pub enabled: bool,
    #[serde(rename = "key")]
    pub smooth_key: Option<String>,
    #[serde(default)]
    pub additional_keys: Vec<String>,
    #[serde(default, rename = "base")]
    pub state_base: String,
    pub shader: Option<String>,
    #[serde(default)]
    pub mode: IconSmoothingMode,
    #[serde(skip)]
    pub update_generation: i32
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub enum IconSmoothingMode {
    #[default]
    Corners,
    CardinalFlags,
    Diagonal,
    NoSprite
}