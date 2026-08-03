use bevy::{prelude::*, reflect::erased_serde::Serializer};
use bitvec::{array::BitArray, order::Msb0};
use microstation_bevy_shared::map::{
    CHUNK_TILES_COUNT, Position, chunk_registry::ChunkRegistry, occupancy::BlocksMovement,
    xy_chunk, xy_index_chunk,
};

#[derive(Component, Default)]
pub struct OccupancyChunk {
    tiles: BitArray<[u8; CHUNK_TILES_COUNT / 8], Msb0>,
}

impl OccupancyChunk {
    pub fn is_blocked(&self, idx: usize) -> bool {
        self.tiles[idx]
    }
    pub fn set_blocked(&mut self, idx: usize, blocked: bool) {
        self.tiles.set(idx, blocked);
    }
}

//вероятно тяжёлое место, хз как его оптимизировать
//TODO надо разбить на несколько функций
// pub fn sync_occupancy(
//     blockers: Query<
//         (&Position, Option<&BlocksMovement>),
//         Or<(Changed<Position>, Changed<BlocksMovement>)>,
//     >,
//     chunk_registry: Res<ChunkRegistry>,
//     mut chunks: Query<&mut OccupancyChunk>,
// ) -> Result<()> {
//     for (world_pos, block) in blockers {
//         let chunk_crd = xy_chunk(world_pos.0);
//         let chunk_idx = xy_index_chunk(chunk_crd);
//         let chunk_e = chunk_registry
//             .chunks
//             .get(&chunk_crd)
//             .ok_or(BevyError::ignore("chunk not exist"))?;
//         let mut chunk = chunks.get_mut(*chunk_e)?;
//         chunk.set_blocked(chunk_idx, block.is_some());
//     }
//     Ok(())
// }

pub fn on_insert_blocker(
    trigger: On<Add, BlocksMovement>,
    blockers: Query<&Position>,
    chunk_registry: Res<ChunkRegistry>,
    mut chunks: Query<&mut OccupancyChunk>,
) -> Result<()> {
    let e = trigger.entity;
    let world_pos = blockers.get(e)?;
    let chunk_crd = xy_chunk(world_pos.0);
    let chunk_idx = xy_index_chunk(world_pos.0);
    let chunk_e = chunk_registry
        .chunks
        .get(&chunk_crd)
        .ok_or(BevyError::ignore("chunk not exist"))?;
    let mut chunk = chunks.get_mut(*chunk_e)?;
    chunk.set_blocked(chunk_idx, true);
    Ok(())
}

pub fn on_remove_blocker(
    trigger: On<Remove, BlocksMovement>,
    blockers: Query<&Position>,
    chunk_registry: Res<ChunkRegistry>,
    mut chunks: Query<&mut OccupancyChunk>,
) -> Result<()> {
    let e = trigger.entity;
    let world_pos = blockers.get(e)?;
    let chunk_crd = xy_chunk(world_pos.0);
    let chunk_idx = xy_index_chunk(world_pos.0);
    let chunk_e = chunk_registry
        .chunks
        .get(&chunk_crd)
        .ok_or(BevyError::ignore("chunk not exist"))?;
    let mut chunk = chunks.get_mut(*chunk_e)?;
    chunk.set_blocked(chunk_idx, false);
    Ok(())
}
