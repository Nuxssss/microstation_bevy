use bevy::prelude::*;
use microstation_bevy_shared::map::tile::TileChunk;

pub fn on_chunk_added(
    trigger: On<Add, TileChunk>,
    chunks: Query<&TileChunk>,
    mut commands: Commands,
) {
    let Ok(chunk) = chunks.get(trigger.entity) else {
        return;
    };
    debug!("chunk {} added at {}", trigger.entity, chunk.crd);
    commands
        .entity(trigger.entity)
        .insert((Transform::default(), Visibility::default()));
}
