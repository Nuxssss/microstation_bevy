use bevy::prelude::*;
use microstation_bevy_shared::{
    grid::Position,
    player::{MoveSpeed, PlayerId},
};

use crate::{network::LocalNetworkId, sprites::tile::TILE_SIZE};

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct PlayerFollower;

pub fn sync_player_sprites(
    players: Query<(&Position, &mut Transform, &MoveSpeed), (With<PlayerId>, With<Position>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (Position(pos), mut transform, speed) in players {
        let target = Vec2::new(pos.x as f32 * 32., pos.y as f32 * 32.);
        let current_pos = transform.translation.truncate();
        let to_target = target - current_pos;
        let distance = to_target.length();

        if distance < 0.5 {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            continue;
        }

        let step = (speed.0 * 30. * dt).min(distance);
        let dir = to_target / distance;
        let new_pos = current_pos + dir * step;

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

pub fn spawn_player(
    trigger: On<Add, PlayerId>,
    players: Query<(&PlayerId, &Position)>,
    mut commands: Commands,
    client_id: Res<LocalNetworkId>,
) {
    let e = trigger.entity;
    let Ok((id, Position(pos))) = players.get(e) else {
        error!("player havent position");
        return;
    };
    let transform = Transform::from_xyz(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 1.0);

    commands.entity(e).insert((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::splat(32.)),
            ..default()
        },
        transform,
        Visibility::default(),
    ));
    if id.0 == client_id.0 {
        commands
            .entity(e)
            .insert((PlayerController, PlayerFollower));
    }
}

pub fn follow_player(
    player: Single<(&Transform, &MoveSpeed), (With<PlayerFollower>, Without<Camera2d>)>,
    camera: Single<&mut Transform, (With<Camera2d>, Without<PlayerFollower>)>,
    time: Res<Time>,
) {
    let target = player.0.translation.truncate();
    let mut camera_transform = camera.into_inner();

    let current = camera_transform.translation.truncate();
    let t = (player.1.0 * 2. * time.delta_secs()).min(1.0);
    let new_pos = current.lerp(target, t);

    camera_transform.translation.x = new_pos.x;
    camera_transform.translation.y = new_pos.y;
}
