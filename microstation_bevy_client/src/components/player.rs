use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct LocalPlayer;
