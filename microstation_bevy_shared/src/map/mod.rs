pub mod chunk_registry;
pub mod occupancy;
pub mod plugin;
pub mod tile;
pub mod tile_registry;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_TILES_COUNT: usize = CHUNK_SIZE.pow(2) as usize;

/// Returns the coordinates of the chunk containing the given tile.
///
/// Chunks are `CHUNK_SIZE x CHUNK_SIZE` tiles. Uses Euclidean division, so
/// negative tile coordinates map to negative chunk coordinates rather than
/// truncating toward zero: tile `(-1, -1)` returns chunk `(-1, -1)`.
///
/// # Examples
/// ```
/// # use bevy::prelude::IVec2;
/// # use microstation_bevy_shared::grid::xy_chunk;
/// assert_eq!(xy_chunk(IVec2::new(5, 10)), IVec2::new(0, 0));
/// assert_eq!(xy_chunk(IVec2::new(32, 0)), IVec2::new(1, 0));
/// assert_eq!(xy_chunk(IVec2::new(-1, 0)), IVec2::new(-1, 0));
/// ```
pub const fn xy_chunk(tile_crd: IVec2) -> IVec2 {
    IVec2::new(
        tile_crd.x.div_euclid(CHUNK_SIZE),
        tile_crd.y.div_euclid(CHUNK_SIZE),
    )
}

/// Returns the flat index of the given tile within its containing chunk's
/// `tiles` array, in the range `0..CHUNK_SIZE * CHUNK_SIZE`.
///
/// The tile's local position within the chunk is computed via Euclidean
/// remainder (staying in range `0..CHUNK_SIZE` on each axis for any world
/// coordinate, including negative ones), then flattened in row-major order:
/// `local_x + local_y * CHUNK_SIZE`.
///
/// This function only computes the index local to a chunk — it does not
/// identify which chunk the tile belongs to (use [`xy_chunk`] for that).
///
/// # Examples
/// ```
/// # use bevy::prelude::IVec2;
/// # use microstation_bevy_shared::grid::xy_index_chunk;
/// assert_eq!(xy_index_chunk(IVec2::new(0, 0)), 0);
/// assert_eq!(xy_index_chunk(IVec2::new(1, 0)), 1);
/// assert_eq!(xy_index_chunk(IVec2::new(0, 1)), 32);
/// assert_eq!(xy_index_chunk(IVec2::new(32, 0)), 0); // wraps to local (0, 0) of next chunk
/// ```
pub const fn xy_index_chunk(tile_crd: IVec2) -> usize {
    let cx = tile_crd.x.rem_euclid(CHUNK_SIZE);
    let cy = tile_crd.y.rem_euclid(CHUNK_SIZE);
    (cx + cy * CHUNK_SIZE) as usize
}

#[derive(Component, Serialize, Deserialize)]
pub struct Position(pub IVec2);
