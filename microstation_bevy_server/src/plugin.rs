use bevy::prelude::*;

use crate::{
    game::GamePlugin,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GamePlugin,
        ));
    }
}