use bevy::prelude::*;

use crate::{
    map::{
        generation::{generate_test_map, spawn_test_walls},
        occupancy::{on_insert_blocker, on_remove_blocker},
        register_tiles,
    },
    plugin::ServerState,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapGeneration;

pub struct MapServerPlugin;

impl Plugin for MapServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ServerState::Round),
            (register_tiles, generate_test_map, spawn_test_walls)
                .chain()
                .in_set(MapGeneration),
        );
        app.add_observer(on_insert_blocker);
        app.add_observer(on_remove_blocker);
    }
}
