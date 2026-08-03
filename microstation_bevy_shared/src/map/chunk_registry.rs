use bevy::{platform::collections::HashMap, prelude::*};

use crate::map::tile::TileChunk;

#[derive(Resource, Default)]
pub struct ChunkRegistry {
    pub chunks: HashMap<IVec2, Entity>,
}

pub fn register_chunk(
    trigger: On<Add, TileChunk>,
    chunks: Query<&TileChunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) -> Result<()> {
    let e = trigger.entity;
    let chunk = chunks.get(e)?;
    chunk_registry.chunks.insert(chunk.crd, e);
    Ok(())
}

pub fn unregister_chunk(
    trigger: On<Remove, TileChunk>,
    chunks: Query<&TileChunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) -> Result<()> {
    let e = trigger.entity;
    let chunk = chunks.get(e)?;
    chunk_registry.chunks.remove(&chunk.crd);
    Ok(())
}
