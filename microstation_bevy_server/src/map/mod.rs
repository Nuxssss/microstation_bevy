use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::replication::diff::WorldDiffExt;
use microstation_bevy_shared::{
    grid::tile_registry::{self, TileRegistry, TileRegistryDiff},
    prototype::{PrototypeKind, PrototypeManager},
};

mod generation;
pub mod plugin;

pub fn register_tiles(prototypes: Res<PrototypeManager>, mut commands: Commands) {
    let mut tile_ids = Vec::from(&["void".to_string()]);
    let ids_from_protos = prototypes.prototypes.iter().filter_map(|(id, proto)| {
        if let PrototypeKind::Tile(_) = &proto.proto {
            Some(id.clone())
        } else {
            None
        }
    });
    tile_ids.extend(ids_from_protos);
    commands.apply_resource_diff::<TileRegistry>(TileRegistryDiff::UpdateTiles(tile_ids));
}
