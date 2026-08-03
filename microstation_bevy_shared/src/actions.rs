use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_event::<PlayerMove>(Channel::Ordered);
    }
}

#[derive(Event, Serialize, Deserialize, Clone, Copy)]
pub struct PlayerMove(pub Vec2);
