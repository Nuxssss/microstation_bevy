use bevy::prelude::*;
use microstation_bevy_shared::{grid::Position, player::PlayerId};

use crate::network::LocalNetworkId;

const CAMERA_FOLLOW_SPEED: f32 = 8.0;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct PlayerFollower;

pub fn sync_player_positions(
    players: Query<(&Position, &mut Transform), (With<PlayerId>, Changed<Position>)>,
) {
    for (Position(pos), mut transform) in players {
        transform.translation.x = pos.x as f32 * 32.;
        transform.translation.y = pos.y as f32 * 32.;
    }
}

pub fn spawn_player(
    trigger: On<Add, PlayerId>,
    players: Query<&PlayerId>,
    mut commands: Commands,
    client_id: Res<LocalNetworkId>,
) {
    let e = trigger.entity;
    commands.entity(e).insert((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::splat(32.)),
            ..default()
        },
        Transform::default(),
        Visibility::default(),
    ));
    let id = players.get(e).unwrap();
    if id.0 == client_id.0 {
        commands
            .entity(e)
            .insert((PlayerController, PlayerFollower));
    }
}

pub fn follow_player(
    player: Single<&Transform, (With<PlayerFollower>, Without<Camera2d>)>,
    camera: Single<&mut Transform, (With<Camera2d>, Without<PlayerFollower>)>,
    time: Res<Time>,
) {
    let target = player.translation.truncate();
    let mut camera_transform = camera.into_inner();

    let current = camera_transform.translation.truncate();
    let t = (CAMERA_FOLLOW_SPEED * time.delta_secs()).min(1.0);
    let new_pos = current.lerp(target, t);

    camera_transform.translation.x = new_pos.x;
    camera_transform.translation.y = new_pos.y;
}
