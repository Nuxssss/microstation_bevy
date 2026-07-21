mod loader;
pub mod plugin;

use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

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
    pub components: HashMap<String, EntityPrototypeComponent>,
}

#[derive(Deserialize)]
pub enum EntityPrototypeComponent {}
