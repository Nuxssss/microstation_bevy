use bevy::{
    log::{Level, LogPlugin},
    utils::default,
};
use clap::Parser;
use microstation_bevy_shared::from_args::Cli;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ServerArgs {
    #[arg(short, long, default_value = "info")]
    log_level: Level,
}

impl Cli<ServerArgs> for LogPlugin {
    fn from_args(args: &ServerArgs) -> Self {
        LogPlugin {
            level: args.log_level,
            ..default()
        }
    }
}
