use crate::components::player::LocalPlayer;
use crate::network::LocalNetworkId;
use crate::{input::InputPlugin, render::RenderPlugin};
use bevy::prelude::*;
use microstation_bevy_shared::components::player::Player;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputPlugin, RenderPlugin));
        app.add_observer(tag_local_player);
    }
}

fn tag_local_player(
    trigger: On<Add, Player>,
    players: Query<&Player>,
    local_id: Res<LocalNetworkId>,
    mut commands: Commands,
) {
    let Ok(player) = players.get(trigger.entity) else {
        return;
    };
    if player.client_id == local_id.0 {
        commands.entity(trigger.entity).insert(LocalPlayer);
    }
    info!("Local player tagged");
}
