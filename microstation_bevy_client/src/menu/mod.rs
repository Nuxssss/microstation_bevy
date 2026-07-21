use crate::network::PendingConnection;
use crate::plugin::ClientState;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy_flair::prelude::*;

#[derive(Component, Default, Clone)]
pub struct MenuRoot;
#[derive(Component, Default, Clone)]
struct LoginField;
#[derive(Component, Default, Clone)]
struct PasswordField;
#[derive(Component, Default, Clone)]
struct AddressField;

#[derive(Event)]
pub struct Connect {
    login: String,
    password: String,
    address: String,
}

pub fn menu() -> impl Scene {
    bsn! {
        #MenuRoot
        DespawnOnExit<ClientState>(ClientState::Menu)
        MenuRoot
        Node
        Styled::StyleSheet("ui/menu.css")
        TabGroup::new(0)
        Children [
            #LoginRow
            Node
            Children [
                #LoginLabel
                Text("Login"),

                input()
                #LoginInput
                LoginField
                TabIndex(1),
            ],
            #PasswordRow
            Node
            Children [
                #PasswordLabel
                Text("Password"),

                input()
                #PasswordInput
                PasswordField
                TabIndex(2),
            ],
            #AddressRow
            Node
            Children[
                #AddressLabel
                Text("Address"),

                input()
                #AddressInput
                AddressField
                TabIndex(3),
            ],
            button()
            #ConnectButton
        ]
    }
}

fn input() -> impl Scene {
    bsn! {
        #Input
        Node
        EditableText
        TextCursorStyle
        TextLayout::no_wrap()
    }
}

fn button() -> impl Scene {
    bsn! {
        Node
        Button
        Hovered
        Children [
            Text::new("Connect")
        ]
        on(|_: On<Pointer<Press>>,
            login_field: Single<&EditableText, With<LoginField>>,
            password_field: Single<&EditableText, With<PasswordField>>,
            address_field: Single<&EditableText, With<AddressField>>,
            mut commands: Commands| {
                commands.trigger(Connect {
                    login: login_field.value().to_string(),
                    password: password_field.value().to_string(),
                    address: address_field.value().to_string(),
                });
        })
    }
}

pub fn on_try_connect(
    connect: On<Connect>,
    mut next_state: ResMut<NextState<ClientState>>,
    mut commands: Commands,
) {
    commands.insert_resource(PendingConnection {
        login: connect.login.clone(),
        password: connect.password.clone(),
        address: connect.address.clone(),
    });
    next_state.set(ClientState::Connecting);
}
