use bevy::prelude::*;

/// Кастомные каналы сверх дефолтных replicon-каналов.
/// Добавляй сюда новые каналы по мере надобности.
pub fn register_channels(_app: &mut App) {
    // Пример: app.add_server_event::<SomeServerEvent>(ChannelKind::Unordered);
    // Дефолтный канал репликации уже создаётся RepliconPlugins.
}

pub const SERVER_ADDR: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 5000;
pub const MAX_CLIENTS: usize = 64;
pub const PROTOCOL_ID: u64 = 0xDEADBEEF_1337_0001;