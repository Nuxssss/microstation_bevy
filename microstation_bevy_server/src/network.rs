use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::shared::backend::connected_client::NetworkId;
use bevy_replicon_renet::RenetServer;
use bevy_replicon_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_replicon_renet::{RepliconRenetPlugins};
use microstation_bevy_shared::protocol::{MAX_CLIENTS, PROTOCOL_ID, SERVER_ADDR, SERVER_PORT};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliconPlugins, RepliconRenetPlugins));

        app.add_systems(Startup, start_server);
        app.add_observer(log_connection);
        app.add_observer(log_disconnection);
    }
}

fn start_server(mut commands: Commands) {
    let bind_addr: SocketAddr = format!("{SERVER_ADDR}:{SERVER_PORT}").parse().unwrap();
    let socket = UdpSocket::bind(bind_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_config = ServerConfig {
        current_time,
        max_clients: MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![bind_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(Default::default());

    commands.insert_resource(server);
    commands.insert_resource(transport);

    info!("Listening {bind_addr}");
}

fn log_connection(trigger: On<Add, ConnectedClient>, network_ids: Query<&NetworkId>) {
    let id = network_ids.get(trigger.entity).unwrap().get();
    info!("client {id} connected!");
}

fn log_disconnection(trigger: On<FromClient<DisconnectRequest>>, network_ids: Query<&NetworkId>) {
    let id = network_ids.get(trigger.client).unwrap().get();
    info!("client {id} disconnected!");
}