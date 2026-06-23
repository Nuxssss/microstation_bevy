use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::backend::connected_client::NetworkId;
use microstation_bevy_shared::world::Position;
use microstation_bevy_shared::{components::player::Player, events::PlayerInput};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player);
        app.add_observer(handle_player_input);
    }
}

const PLAYER_SPEED: f32 = 200.0;

fn spawn_player(
    trigger: On<Add, ConnectedClient>,
    network_ids: Query<&NetworkId>,
    mut commands: Commands,
) {
    let id = network_ids.get(trigger.entity).unwrap().get();
    commands
        .entity(trigger.entity)
        .insert(Player { client_id: id })
        .insert(Position(IVec2::ZERO))
        .insert(Replicated);
    info!("Spawned player {id}");
}

fn handle_player_input(
    trigger: On<FromClient<PlayerInput>>,
    _time: Res<Time>,
    mut player_positions: Query<&mut Position>,
) {
    let ClientId::Client(e) = trigger.client_id else {
        return;
    };
    let mut pos = player_positions.get_mut(e).unwrap();
    pos.0 += trigger.direction;
}
