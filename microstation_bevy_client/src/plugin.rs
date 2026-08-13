use crate::cli::ClientArgs;
use crate::input::InputPlugin;
use crate::map::plugin::MapClientPlugin;
use crate::menu::{menu, on_try_connect};
use crate::network::{NetworkClientPlugin, PendingConnection};
use crate::player::{follow_player, spawn_player, sync_player_sprites};
use crate::prototype::PrototypeClientPlugin;
use crate::sprites::plugin::GameSpritesPlugin;
use bevy::prelude::*;
use microstation_bevy_shared::from_args::Cli;
use microstation_bevy_shared::prototype::plugin::PrototypeLoad;

pub struct ClientPlugin {
    skip_menu: bool,
    address: String,
}

impl Cli<ClientArgs> for ClientPlugin {
    fn from_args(args: &ClientArgs) -> Self {
        Self {
            skip_menu: args.connect,
            address: args.address.clone(),
        }
    }
}

#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum ClientState {
    #[default]
    Menu,
    Connecting,
    Lobby,
    InGame,
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>();
        if self.skip_menu {
            app.insert_state(ClientState::Connecting);
            app.insert_resource(PendingConnection {
                login: "John Doe".into(),
                password: "aboba123".into(),
                address: self.address.clone(),
            });
        }

        app.add_plugins(NetworkClientPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(GameSpritesPlugin);
        app.add_plugins(PrototypeClientPlugin);
        app.add_plugins(MapClientPlugin);
        app.configure_sets(Startup, (PrototypeLoad,));
        app.add_systems(Startup, camera_setup);
        app.add_observer(on_try_connect.run_if(in_state(ClientState::Menu)));
        app.add_systems(OnEnter(ClientState::Menu), menu.spawn());
        app.add_systems(
            Update,
            (sync_player_sprites.run_if(in_state(ClientState::InGame)),),
        );
        app.add_systems(Update, follow_player.run_if(in_state(ClientState::InGame)));
        app.add_observer(spawn_player.run_if(in_state(ClientState::InGame)));
    }
}
