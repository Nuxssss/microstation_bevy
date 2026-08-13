// microstation_bevy_client/src/sprites/chunk.rs
use bevy::prelude::*;
use microstation_bevy_shared::map::CHUNK_SIZE;
use microstation_bevy_shared::map::tile::TileChunk;

use crate::sprites::tile::TileSprites;

pub const TILE_SIZE: f32 = 32.0;

pub fn sync_chunk_sprites(
    mut commands: Commands,
    chunks: Query<(Entity, &TileChunk), Changed<TileChunk>>,
    tile_sprites: Res<TileSprites>,
) {
    for (chunk_entity, chunk) in &chunks {
        commands.entity(chunk_entity).despawn_children();

        let base_world = IVec2::new(chunk.crd.x * CHUNK_SIZE, chunk.crd.y * CHUNK_SIZE);

        for (local_idx, &tile_idx) in chunk.tiles.iter().enumerate() {
            if tile_idx == 0 {
                continue; // void tile
            }
            let Some(image_handle) = tile_sprites.sprite_by_idx(tile_idx) else {
                error_once!("no tile with index {} in tile cache", tile_idx);
                error_once!("tile cache: {:?}", &tile_sprites);
                continue;
            };

            let local_pos =
                IVec2::new(local_idx as i32 % CHUNK_SIZE, local_idx as i32 / CHUNK_SIZE);

            let world_pos = base_world + local_pos;

            commands.entity(chunk_entity).with_child((
                Sprite {
                    image: image_handle.clone(),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    world_pos.x as f32 * TILE_SIZE,
                    world_pos.y as f32 * TILE_SIZE,
                    0.,
                ),
                Visibility::default(),
            ));
        }
    }
}
