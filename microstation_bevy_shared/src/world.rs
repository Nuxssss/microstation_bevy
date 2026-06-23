use bevy::math::IVec2;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug)]
#[require(Transform, Visibility)]
pub struct Position(pub IVec2);

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct ChunkPosition(pub IVec2);
