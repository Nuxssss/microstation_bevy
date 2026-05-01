
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Component, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Item {
    #[serde_inline_default("Small".to_string())]
    pub size: String,
    //TODO add other fields
}