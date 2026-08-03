mod cli;
mod map;
mod network;
mod player;
mod plugin;
pub mod prototype;

use crate::cli::ServerArgs;
use crate::plugin::ServerPlugin;
use bevy::ecs;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use clap::Parser;
use microstation_bevy_shared::from_args::Cli;
use microstation_bevy_shared::plugin::SharedPlugin;

fn main() -> AppExit {
    let args = ServerArgs::parse();
    App::new()
        .set_error_handler(ecs::error::error)
        .add_plugins(MinimalPlugins)
        .add_plugins(StatesPlugin)
        .add_plugins(LogPlugin {
            ..Cli::from_args(&args)
        })
        .add_plugins(ServerPlugin)
        .add_plugins(SharedPlugin)
        .run()
}
