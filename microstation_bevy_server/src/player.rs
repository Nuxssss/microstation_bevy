use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::backend::connected_client::NetworkId;
use microstation_bevy_shared::{
    map::Position,
    player::{MoveSpeed, PlayerId},
};

#[derive(Component)]
pub struct PlayerController(pub Entity);

#[derive(Component)]
pub struct MoveCooldown(pub Timer);

//TODO: пока что игрок спавнится сразу же когда клиент подключается к этоуй хуйне
// и это надо переделать так чтобы функция спавна реагировало на событие создания игрока или что то вроде
// которое я хуй знает где должно отправляться при переходе в лобби наверное не знаю, потом сделаю
pub fn spawn_player(
    trigger: On<Add, ConnectedClient>,
    network_ids: Query<&NetworkId>,
    mut commands: Commands,
) -> Result<()> {
    let id = network_ids.get(trigger.entity)?.get();
    let speed = 10.;
    let entity = commands
        .spawn((
            PlayerId(id),
            Position((0, 0).into()),
            MoveSpeed(speed),
            MoveCooldown(Timer::from_seconds(1.0 / speed, TimerMode::Once)),
            Replicated,
        ))
        .id();
    commands
        .entity(trigger.entity)
        .insert(PlayerController(entity));
    debug!("spawned player: {}", entity);
    Ok(())
}

pub fn move_cooldown_tick(cooldowns: Query<&mut MoveCooldown>, time: Res<Time>) {
    for mut cooldown in cooldowns {
        cooldown.0.tick(time.delta());
    }
}
