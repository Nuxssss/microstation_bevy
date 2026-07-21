use bevy::prelude::*;

use crate::{
    map::{generation::generate_test_map, register_tiles},
    plugin::ServerState,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapGeneration;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ServerState::Round),
            (register_tiles, generate_test_map)
                .chain()
                .in_set(MapGeneration),
        );
    }
}
