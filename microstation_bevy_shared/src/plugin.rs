use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::{
    events::PlayerInput,
    protocol,
};
use crate::components::icon_smooth::IconSmooth;
use crate::components::item::Item;
use crate::components::player::Player;
use crate::components::sprite::{ComplexSprite, SpriteReplicated};
use crate::prototypes::loader::load_prototypes;
use crate::prototypes::PrototypeManager;
use crate::world::Position;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        protocol::register_channels(app);

        app.init_resource::<PrototypeManager>();
        app.add_systems(Startup, load_prototypes);
        app.replicate::<Position>();
        app.replicate::<Player>();
        app.replicate::<IconSmooth>();
        app.replicate::<Item>();
        app.replicate_as::<ComplexSprite, SpriteReplicated>();
        app.add_client_event::<PlayerInput>(Channel::Ordered);
    }
}