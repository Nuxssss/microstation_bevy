use bevy::prelude::*;
use microstation_bevy_shared::{
    grid::tile_registry::{self, TileRegistry},
    prototype::{PrototypeKind, PrototypeManager},
};
use std::collections::HashMap;

pub const TILE_SIZE: f32 = 32.;

#[derive(Resource, Debug, Default)]
pub struct TileSprites {
    pub sprites: Vec<Handle<Image>>,
    pub indexes: HashMap<u8, usize>,
}

impl TileSprites {
    pub fn sprite_by_idx(&self, tile_idx: u8) -> Option<&Handle<Image>> {
        let sprites_idx = self.indexes.get(&tile_idx)?;
        self.sprites.get(*sprites_idx)
    }
    pub fn clear(&mut self) {
        self.sprites.clear();
        self.indexes.clear();
    }
}

pub fn reload_tile_sprites(
    prototypes: Res<PrototypeManager>,
    tile_registry: Res<TileRegistry>,
    mut tile_sprites: ResMut<TileSprites>,
    asset_server: Res<AssetServer>,
) {
    tile_sprites.clear();
    for (tile_idx, tile_id) in tile_registry.tiles().enumerate() {
        if tile_idx == 0 {
            continue;
        };
        let Some(proto) = prototypes.prototypes.get(tile_id) else {
            warn!("no prototype for tile_id={tile_id}");
            continue;
        };
        let PrototypeKind::Tile(tile) = &proto.proto else {
            continue;
        };
        let image = asset_server.load(tile.sprite_path.clone());
        let idx = tile_sprites.sprites.len();
        tile_sprites.indexes.insert(tile_idx as u8, idx);
        tile_sprites.sprites.push(image);
    }
    debug!("reloaded tile sprites");
}
