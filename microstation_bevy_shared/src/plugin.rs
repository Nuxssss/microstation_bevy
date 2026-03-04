use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::RepliconSharedPlugin;
use bevy_replicon_renet::RepliconRenetPlugins;
use crate::{
    components::player::PlayerPosition,
    events::PlayerInput,
    protocol,
};

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        protocol::register_channels(app);

        // Компоненты, которые сервер реплицирует клиентам
        app.replicate::<PlayerPosition>();

        // Клиентские события, которые сервер получает
        app.add_client_event::<PlayerInput>(Channel::Ordered);
    }
}