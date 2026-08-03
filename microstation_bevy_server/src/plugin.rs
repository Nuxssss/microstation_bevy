use std::time::Duration;

use crate::map::occupancy::OccupancyChunk;
use crate::map::plugin::MapServerPlugin;
use crate::player::{MoveCooldown, PlayerController, move_cooldown_tick};
use crate::{network::NetworkServerPlugin, player::spawn_player};
use bevy::prelude::*;
use bevy_replicon::prelude::{ClientId, FromClient};
use microstation_bevy_shared::actions::PlayerMove;
use microstation_bevy_shared::map::chunk_registry::ChunkRegistry;
use microstation_bevy_shared::map::{Position, xy_chunk, xy_index_chunk};
use microstation_bevy_shared::player::MoveSpeed;
use microstation_bevy_shared::prototype::plugin::PrototypeLoad;
// сервер - потому что все системы, запускающиеся здесь, специфичны для сервера

#[derive(States, Debug, Hash, Clone, Eq, PartialEq, Default)]
pub enum ServerState {
    Round,
    #[default]
    Waiting,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ServerState>();
        app.add_plugins(NetworkServerPlugin);
        app.add_plugins(MapServerPlugin);
        app.configure_sets(Startup, (PrototypeLoad,));
        app.add_systems(Update, skip_waiting.run_if(in_state(ServerState::Waiting)));
        app.add_systems(Update, move_cooldown_tick);
        app.add_observer(spawn_player.run_if(in_state(ServerState::Round))); //TODO Убрать/переделать когда будет сделано лобби
        app.add_observer(on_move);
    }
}

fn skip_waiting(mut next_state: ResMut<NextState<ServerState>>) {
    info!("skipping waiting state by default");
    next_state.set(ServerState::Round);
}

fn on_move(
    trigger: On<FromClient<PlayerMove>>,
    controllers: Query<&PlayerController>,
    mut positions: Query<(&mut Position, &mut MoveCooldown, &MoveSpeed)>,
    chunk_registry: Res<ChunkRegistry>,
    chunks: Query<&OccupancyChunk>,
) -> Result<()> {
    let ClientId::Client(e) = trigger.client_id else {
        return Err(BevyError::warning(
            "the move action isn't triggered by the client",
        ));
    };
    let PlayerController(e) = controllers.get(e)?;

    let (mut pos, mut cooldown, speed) = positions.get_mut(*e)?;
    if !cooldown.0.is_finished() {
        return Ok(());
    }
    let target_pos = pos.0 + trigger.0.as_ivec2();
    let chunk_crd = xy_chunk(target_pos);
    let chunk_idx = xy_index_chunk(target_pos);
    let &chunk_entity = chunk_registry
        .chunks
        .get(&chunk_crd)
        .ok_or(BevyError::error("chunk not exist"))?;
    let chunk = chunks.get(chunk_entity)?;
    if chunk.is_blocked(chunk_idx) {
        return Ok(());
    }
    pos.0 = target_pos;
    cooldown
        .0
        .set_duration(Duration::from_secs_f32(1. / speed.0 * trigger.0.length()));
    cooldown.0.reset();
    Ok(())
}
