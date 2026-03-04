use bevy::prelude::*;
use crate::{
    input::InputPlugin,
    render::RenderPlugin,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputPlugin,
            RenderPlugin,
        ));
    }
}