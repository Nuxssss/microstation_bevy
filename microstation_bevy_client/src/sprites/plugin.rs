use bevy::prelude::*;
use microstation_bevy_shared::grid::tile_registry::TileRegistry;

use crate::sprites::chunk::on_chunk_added;

use super::{
    chunk::sync_chunk_sprites,
    tile::{TileSprites, reload_tile_sprites},
};

pub struct GameSpritesPlugin;

impl Plugin for GameSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileSprites>();
        app.add_systems(Update, sync_chunk_sprites);
        app.add_systems(
            Update,
            reload_tile_sprites.run_if(resource_changed::<TileRegistry>),
        );
        app.add_observer(on_chunk_added);
    }
}
