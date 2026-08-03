use crate::{
    actions::ActionPlugin,
    map::plugin::GridPlugin,
    player::{MoveSpeed, PlayerId},
    prototype::plugin::PrototypePlugin,
};
use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PrototypePlugin);
        app.add_plugins(ActionPlugin);
        app.add_plugins(GridPlugin);
        app.replicate::<PlayerId>();
        app.replicate::<MoveSpeed>();
        app.replicate::<Name>();
    }
}
