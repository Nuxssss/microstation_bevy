use bevy::prelude::*;
use bevy_replicon::prelude::*;
use microstation_bevy_shared::{
    map::tile_registry::{TileRegistry, TileRegistryDiff},
    prototype::{PrototypeKind, PrototypeManager},
};

mod generation;
pub mod occupancy;
pub mod plugin;

pub fn register_tiles(prototypes: Res<PrototypeManager>, mut commands: Commands) {
    let mut tile_ids = vec!["void".to_string()];
    tile_ids.extend(prototypes.prototypes.iter().filter_map(|(id, proto)| {
        matches!(&proto.proto, PrototypeKind::Tile(_)).then(|| id.clone())
    }));
    commands.apply_resource_diff::<TileRegistry>(TileRegistryDiff::UpdateTiles(tile_ids));
}
