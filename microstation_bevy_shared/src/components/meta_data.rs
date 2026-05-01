use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
}
