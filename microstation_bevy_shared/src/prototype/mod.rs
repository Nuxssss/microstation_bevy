mod loader;
pub mod plugin;

use bevy::prelude::*;
use bevy_replicon::shared::replication::Replicated;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::map::{Position, occupancy::BlocksMovement};

/// Component that represent the ID of the prototype from which this entity was created
#[derive(Component, Serialize, Deserialize)]
pub struct PrototypeId(pub String);

#[derive(Resource, Default)]
pub struct PrototypeManager {
    pub prototypes: HashMap<String, Prototype>,
}

#[derive(Deserialize)]
pub struct Prototype {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "parent")]
    pub parent_id: Option<String>,
    #[serde(flatten)]
    pub proto: PrototypeKind,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PrototypeKind {
    Entity(EntityPrototype),
    Tile(TilePrototype),
}
#[derive(Deserialize)]
pub struct TilePrototype {
    #[serde(rename = "sprite")]
    pub sprite_path: String,
}

#[derive(Deserialize)]
pub struct EntityPrototype {
    pub components: Vec<EntityPrototypeComponent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EntityPrototypeComponent {
    Sprite(SpriteInfo),
    BlocksMovement,
}

#[derive(Deserialize, Serialize)]
pub struct SpriteInfo {
    pub path: String,
    pub mode: SpriteMode,
}

#[derive(Deserialize, Serialize, Default)]
pub enum SpriteMode {
    #[default]
    Simple,
    Wall,
}
