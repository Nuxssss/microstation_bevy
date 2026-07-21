use bevy::{platform::collections::HashMap, prelude::*};
use bevy_replicon::{
    prelude::Diffable,
    shared::replication::{Replicated, signature::Signature},
};
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Debug)]
#[require(Signature::from("TileRegistry"))]
pub struct TileRegistry {
    tiles: Vec<String>,
    tiles_ids: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
pub enum TileRegistryDiff {
    NewTile(String),
    UpdateTiles(Vec<String>),
}

impl Diffable for TileRegistry {
    type Diff = TileRegistryDiff;

    fn apply_diff(&mut self, diff: &Self::Diff) -> Result<()> {
        debug!("applied diff for tileregistry");
        match diff {
            TileRegistryDiff::NewTile(tile) => {
                let tile = tile.clone();
                if self.tiles_ids.contains_key(&tile) {
                    return Ok(());
                }
                self.tiles_ids.insert(tile.clone(), self.tiles.len());
                self.tiles.push(tile);
            }
            TileRegistryDiff::UpdateTiles(items) => {
                self.tiles_ids.clear();
                self.tiles = items.clone();
                for (i, tile_id) in self.tiles.iter().enumerate() {
                    self.tiles_ids.insert(tile_id.clone(), i);
                }
            }
        }
        Ok(())
    }
}

impl TileRegistry {
    pub fn name_of(&self, id: u8) -> Option<&str> {
        self.tiles.get(id as usize).map(String::as_str)
    }

    pub fn id_of(&self, name: &str) -> Option<u8> {
        self.tiles_ids.get(name).map(|&i| i as u8)
    }
    pub fn tiles(&self) -> impl Iterator<Item = &String> {
        self.tiles.iter()
    }
}

impl Default for TileRegistry {
    fn default() -> Self {
        let mut tiles = Vec::new();
        let mut tiles_ids = HashMap::new();
        tiles.push("void".to_string()); // id 0 = пусто, зарезервировано
        tiles_ids.insert("void".to_string(), 0);
        Self { tiles, tiles_ids }
    }
}
