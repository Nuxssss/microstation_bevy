use bevy::prelude::*;

use crate::sprites::{
    tile::{TileSpritePlugin, TileSprites},
    wall::WallSpritePlugin,
};

use super::chunk::sync_chunk_sprites;

pub struct GameSpritesPlugin;

impl Plugin for GameSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WallSpritePlugin);
        app.add_plugins(TileSpritePlugin);
        app.add_systems(
            Update,
            sync_chunk_sprites
                .run_if(|tile_sprites: Res<TileSprites>| !tile_sprites.sprites.is_empty()),
        );
    }
}
