use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::map::CHUNK_TILES_COUNT;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct TileChunk {
    #[serde(
        serialize_with = "tiles_serialize",
        deserialize_with = "tiles_deserialize"
    )]
    pub tiles: [u8; CHUNK_TILES_COUNT],
    pub crd: IVec2,
}

#[derive(Serialize, Deserialize)]
pub enum TileChunkDiff {
    OneTile { idx: usize, tile: u8 },
    NewChunk(Box<TileChunk>),
}

impl Diffable for TileChunk {
    type Diff = TileChunkDiff;

    fn apply_diff(&mut self, diff: &Self::Diff) -> Result<()> {
        match diff {
            TileChunkDiff::OneTile { idx, tile } => {
                self.tiles[*idx] = *tile;
            }
            TileChunkDiff::NewChunk(chunk) => {
                *self = *chunk.clone();
            }
        }
        Ok(())
    }
}

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
