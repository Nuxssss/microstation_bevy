use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::{
    Position,
    chunk_registry::{ChunkRegistry, register_chunk, unregister_chunk},
    tile_registry::TileRegistry,
};
use crate::map::tile::TileChunk;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkRegistry>();
        app.replicate_diff::<TileChunk>();
        app.replicate_resource::<TileRegistry>();
        app.init_resource::<TileRegistry>();
        app.replicate::<Position>();
        app.add_observer(register_chunk);
        app.add_observer(unregister_chunk);
    }
}
