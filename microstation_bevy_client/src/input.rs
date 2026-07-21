use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
use microstation_bevy_shared::actions;

use crate::{player::PlayerController, plugin::ClientState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<OnFoot>();
        app.add_observer(on_move.run_if(in_state(ClientState::InGame)));
        app.add_observer(add_actions_and_bindings.run_if(in_state(ClientState::InGame)));
    }
}

pub fn add_actions_and_bindings(trigger: On<Add, PlayerController>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        // Тут возможно нужно будет потом добавить INACTIVE но я не понимаю как это правильно сделать
        OnFoot,
        actions!(
            OnFoot[(
                Action::<Move>::new(),
                Bindings::spawn(Cardinal::wasd_keys())
            )]
        ),
    ));
}

pub fn on_move(trigger: On<Fire<Move>>, mut commands: Commands) {
    commands.client_trigger(actions::Move(trigger.value));
}

#[derive(Component)]
pub struct OnFoot;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;
