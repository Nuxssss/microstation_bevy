use crate::prototype::PrototypeManager;
use crate::prototype::loader::load_prototypes;
use bevy::app::App;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrototypeLoad;
pub struct PrototypePlugin;

impl Plugin for PrototypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrototypeManager>();
        app.add_systems(Startup, load_prototypes.in_set(PrototypeLoad));
    }
}
