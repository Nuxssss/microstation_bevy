mod plugin;
mod network;
mod input;
mod render;

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::window::WindowResolution;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::RepliconRenetPlugins;
use plugin::ClientPlugin;
use microstation_bevy_shared::plugin::SharedPlugin;
use crate::network::NetworkClientPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Microstation — Client".into(),
                resolution: WindowResolution::default(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(NetworkClientPlugin)
        .add_plugins(SharedPlugin)
        .add_plugins(ClientPlugin)
        .run();
}