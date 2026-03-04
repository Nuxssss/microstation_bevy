mod plugin;
mod network;
mod game;

use bevy::ecs::schedule::LogLevel;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use microstation_bevy_shared::plugin::SharedPlugin;
use plugin::ServerPlugin;
use crate::network::NetworkServerPlugin;

fn main() {
    App::new()
        // MinimalPlugins — нет окна, нет рендера, нет звука
        .add_plugins(MinimalPlugins)
        .add_plugins(StatesPlugin)
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            custom_layer: |_| None,
            fmt_layer: |_| None,
        })
        .add_plugins(NetworkServerPlugin)
        .add_plugins(SharedPlugin)
        .add_plugins(ServerPlugin)
        .run();
}