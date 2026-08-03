use bevy::prelude::*;

use crate::map::chunk::on_chunk_added;

pub struct MapClientPlugin;

impl Plugin for MapClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_chunk_added);
    }
}
