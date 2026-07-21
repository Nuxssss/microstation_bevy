use bevy::{platform::collections::HashMap, prelude::*};

use super::Chunk;

#[derive(Resource, Default)]
pub struct ChunkRegistry {
    pub chunks: HashMap<IVec2, Entity>,
}

pub fn register_chunk(
    trigger: On<Add, Chunk>,
    chunks: Query<&Chunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let e = trigger.entity;
    let chunk = chunks.get(e).unwrap();
    chunk_registry.chunks.insert(chunk.crd, e);
}

pub fn unregister_chunk(
    trigger: On<Remove, Chunk>,
    chunks: Query<&Chunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let e = trigger.entity;
    let chunk = chunks.get(e).unwrap();
    chunk_registry.chunks.remove(&chunk.crd);
}
