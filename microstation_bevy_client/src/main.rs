mod plugin;
mod network;
mod input;
mod render;
mod components;
mod rsi;

use bevy::ecs;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use plugin::ClientPlugin;
use microstation_bevy_shared::plugin::SharedPlugin;
use crate::network::NetworkClientPlugin;
use crate::rsi::RsiPlugin;

fn main() -> AppExit {
    App::new()
        .set_error_handler(ecs::error::error)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Client".into(),
                resolution: WindowResolution::default(),
                ..default()
            }),
            ..default()
        })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
            file_path: "../Resources".to_string(),
            ..default()
        }))
        .add_plugins(NetworkClientPlugin)
        .add_plugins(SharedPlugin)
        .add_plugins(ClientPlugin)
        .add_plugins(RsiPlugin)
        .run()
}