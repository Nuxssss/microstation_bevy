use bevy::prelude::*;
use microstation_bevy_shared::components::player::PlayerPosition;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, sync_player_sprites);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Синхронизирует Transform со значениями, пришедшими от сервера через репликацию.
fn sync_player_sprites(
    mut query: Query<(&PlayerPosition, &mut Transform), Changed<PlayerPosition>>,
) {
    for (pos, mut transform) in &mut query {
        transform.translation = pos.0.extend(0.0);
    }
}