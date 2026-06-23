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

fn send_input(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    let mut direction = IVec2::ZERO;
    direction.y += keys.pressed(KeyCode::ArrowUp) as i32;
    direction.y -= keys.pressed(KeyCode::ArrowDown) as i32;
    direction.x += keys.pressed(KeyCode::ArrowRight) as i32;
    direction.x -= keys.pressed(KeyCode::ArrowLeft) as i32;

    if direction != IVec2::ZERO {
        commands.client_trigger(PlayerInput { direction });
    }
}
