mod chunk_registry;
pub mod plugin;
pub mod tile_registry;

use bevy::prelude::*;
use bevy_replicon::prelude::Diffable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_TILES_COUNT: usize = CHUNK_SIZE.pow(2) as usize;

fn tiles_serialize<S: Serializer>(
    bytes: &[u8; CHUNK_TILES_COUNT],
    s: S,
) -> Result<S::Ok, S::Error> {
    serde_bytes::serialize(bytes.as_slice(), s)
}

fn tiles_deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; CHUNK_TILES_COUNT], D::Error> {
    let vec = serde_bytes::deserialize::<Vec<u8>, D>(d)?;
    vec.try_into()
        .map_err(|_| serde::de::Error::custom("wrong length"))
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Chunk {
    #[serde(
        serialize_with = "tiles_serialize",
        deserialize_with = "tiles_deserialize"
    )]
    pub tiles: [u8; CHUNK_TILES_COUNT],
    pub crd: IVec2,
}

#[derive(Serialize, Deserialize)]
pub enum ChunkDiff {
    OneTile { idx: usize, tile: u8 },
    NewChunk(Chunk),
}

impl Diffable for Chunk {
    type Diff = ChunkDiff;

    fn apply_diff(&mut self, diff: &Self::Diff) -> Result<()> {
        match diff {
            ChunkDiff::OneTile { idx, tile } => {
                self.tiles[*idx] = *tile;
            }
            ChunkDiff::NewChunk(chunk) => {
                *self = chunk.clone();
            }
        }
        Ok(())
    }
}

pub const fn xy_chunk(tile_crd: IVec2) -> IVec2 {
    IVec2::new(
        tile_crd.x.div_euclid(CHUNK_SIZE),
        tile_crd.y.div_euclid(CHUNK_SIZE),
    )
}

pub const fn xy_index_chunk(tile_crd: IVec2) -> usize {
    let cx = tile_crd.x.rem_euclid(CHUNK_SIZE);
    let cy = tile_crd.y.rem_euclid(CHUNK_SIZE);
    (cx + cy * CHUNK_SIZE) as usize
}

#[derive(Component, Serialize, Deserialize)]
pub struct Position(pub IVec2);
