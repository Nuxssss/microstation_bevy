use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::backend::connected_client::NetworkId;
use microstation_bevy_shared::{grid::Position, player::PlayerId};

#[derive(Component)]
pub struct PlayerController(pub Entity);

//TODO: пока что игрок спавнится сразу же когда клиент подключается к этоуй хуйне
// и это надо переделать так чтобы функция спавна реагировало на событие создания игрока или что то вроде
// которое я хуй знает где должно отправляться при переходе в лобби наверное не знаю, потом сделаю
pub fn spawn_player(
    trigger: On<Add, ConnectedClient>,
    network_ids: Query<&NetworkId>,
    mut commands: Commands,
) {
    let Ok(id) = network_ids.get(trigger.entity).map(|x| x.get()) else {
        return;
    };
    let entity = commands
        .spawn((PlayerId(id), Position((0, 0).into()), Replicated))
        .id();
    commands
        .entity(trigger.entity)
        .insert(PlayerController(entity));
    debug!("spawned player: {}", entity);
}
