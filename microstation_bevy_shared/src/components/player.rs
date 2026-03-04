use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Маркер: эта сущность — игрок.
#[derive(Component)]
pub struct Player {
    /// ID клиента из bevy_replicon
    pub client_id: u64,
}

/// Реплицируемая позиция игрока (отдельно от Transform, чтобы контролировать что именно летит по сети)
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerPosition(pub Vec2);