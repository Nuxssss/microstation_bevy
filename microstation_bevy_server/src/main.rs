mod plugin;
mod network;
mod game;
mod console;

use bevy::ecs;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use microstation_bevy_shared::plugin::SharedPlugin;
use plugin::ServerPlugin;
use crate::network::NetworkServerPlugin;

fn main() -> AppExit {
    App::new()
        .set_error_handler(ecs::error::error)
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
        .run()
}