use bevy::prelude::*;

use crate::{
    game::GamePlugin,
};
use crate::console::ConsolePlugin;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GamePlugin,
            ConsolePlugin { port: 5001 }
        ));
    }
}