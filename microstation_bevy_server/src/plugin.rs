use std::time::Duration;

use crate::map::plugin::MapPlugin;
use crate::player::{MoveCooldown, PlayerController, move_cooldown_tick};
use crate::{network::NetworkServerPlugin, player::spawn_player};
use bevy::prelude::*;
use bevy_replicon::prelude::{ClientId, FromClient};
use microstation_bevy_shared::actions::Move;
use microstation_bevy_shared::grid::Position;
use microstation_bevy_shared::player::MoveSpeed;
use microstation_bevy_shared::prototype::plugin::PrototypeLoad;
// сервер - потому что все системы, запускающиеся здесь, специфичны для сервера

#[derive(States, Debug, Hash, Clone, Eq, PartialEq, Default)]
pub enum ServerState {
    Round, //пока что тут пусть
    #[default]
    Waiting,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ServerState>();
        app.add_plugins(NetworkServerPlugin);
        app.add_plugins(MapPlugin);
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
    trigger: On<FromClient<Move>>,
    controllers: Query<&PlayerController>,
    mut positions: Query<(&mut Position, &mut MoveCooldown, &MoveSpeed)>,
) {
    let ClientId::Client(e) = trigger.client_id else {
        warn!("the move action isn't triggered by the client");
        return;
    };
    let Ok(PlayerController(e)) = controllers.get(e) else {
        return;
    };

    let Ok((mut pos, mut cooldown, speed)) = positions.get_mut(*e) else {
        warn!("player {} haven't position component", e);
        return;
    };
    if !cooldown.0.is_finished() {
        return;
    }
    pos.0 += trigger.0.as_ivec2();
    cooldown
        .0
        .set_duration(Duration::from_secs_f32(1. / speed.0 * trigger.0.length()));
    cooldown.0.reset();
}
