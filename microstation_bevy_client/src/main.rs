mod cli;
mod input;
pub mod map;
pub mod menu;
mod network;
mod player;
mod plugin;
pub mod prototype;
mod sprites;

use crate::cli::ClientArgs;
use crate::plugin::ClientPlugin;
use bevy::ecs;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_flair::FlairPlugin;
use clap::Parser;
use microstation_bevy_shared::from_args::Cli;
use microstation_bevy_shared::plugin::SharedPlugin;

fn main() -> AppExit {
    let args = ClientArgs::parse();

    App::new()
        .set_error_handler(ecs::error::error)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Client".into(),
                        resolution: WindowResolution::default(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../assets".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    ..Cli::from_args(&args)
                }),
        )
        .add_plugins(TabNavigationPlugin)
        .add_plugins(FlairPlugin)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins(ClientPlugin::from_args(&args))
        .add_plugins(SharedPlugin)
        .run()
}
