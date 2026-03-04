use bevy::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
use bevy_replicon_renet::client_connected;
use microstation_bevy_shared::events::PlayerInput;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_input.run_if(client_connected));
    }
}

/// Читает клавиатуру и отправляет PlayerInput на сервер через replicon.
fn send_input(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    let mut direction = Vec2::ZERO;
    let mut send = false;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        send = true;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        send = true;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        send = true;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        send = true;
    }
    if send {
        commands.client_trigger(
            PlayerInput {
                direction: direction.normalize_or_zero(),
            }
        );
    }
}
