use crate::prototype::loader::load_prototypes;
use crate::prototype::{PrototypeId, PrototypeManager};
use bevy::app::App;
use bevy::prelude::*;
use bevy_replicon::shared::replication::rules::AppRuleExt;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrototypeLoad;
pub struct PrototypePlugin;

impl Plugin for PrototypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrototypeManager>();
        app.replicate::<PrototypeId>();
        app.add_systems(Startup, load_prototypes.in_set(PrototypeLoad));
    }
}
