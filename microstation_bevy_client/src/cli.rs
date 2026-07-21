use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use clap::Parser;
use microstation_bevy_shared::from_args::Cli;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs {
    #[arg(short, long, default_value = "127.0.0.1:5000")]
    pub address: String,
    #[arg(short, long)]
    pub connect: bool,
    #[arg(short, long, default_value = "info")]
    log_level: Level,
}

impl Cli<ClientArgs> for LogPlugin {
    fn from_args(args: &ClientArgs) -> Self {
        LogPlugin {
            level: args.log_level,
            ..default()
        }
    }
}
