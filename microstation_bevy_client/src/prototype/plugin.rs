use bevy::prelude::*;

use crate::prototype::on_prototype_added;

pub struct PrototypeClientPlugin;

impl Plugin for PrototypeClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_prototype_added);
    }
}
