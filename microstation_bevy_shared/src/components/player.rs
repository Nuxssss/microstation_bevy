use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub client_id: u64,
}
