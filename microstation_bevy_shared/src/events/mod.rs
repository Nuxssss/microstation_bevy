use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Ввод игрока — клиент отправляет на сервер.
/// Добавляй новые поля по мере роста — структура позволяет расширять без боли.
#[derive(Event, Serialize, Deserialize, Clone, Debug, Default)]
pub struct PlayerInput {
    pub direction: IVec2,
}
