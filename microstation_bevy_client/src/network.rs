use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_replicon_renet::{RenetClient, RepliconRenetPlugins};

use crate::plugin::ClientState;
use microstation_bevy_shared::protocol::PROTOCOL_ID;

#[derive(Resource)]
pub struct PendingConnection {
    pub login: String,
    pub password: String,
    pub address: String,
}

pub struct NetworkClientPlugin;
#[derive(Resource)]
pub struct LocalNetworkId(pub u64);

impl Plugin for NetworkClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins);
        app.add_plugins(RepliconRenetPlugins);

        app.add_systems(OnEnter(ClientState::Connecting), connect_to_server);
    }
}

fn connect_to_server(
    mut commands: Commands,
    connect: Res<PendingConnection>,
    mut next_state: ResMut<NextState<ClientState>>,
) -> Result<()> {
    let server_addr: SocketAddr = connect.address.parse()?;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

    let cid = current_time.as_millis() as u64;

    //TODO: сделать аутентификацию по токену, вместо случайного client_id
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id: cid,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;
    let client = RenetClient::new(Default::default());

    commands.insert_resource(client);
    commands.insert_resource(transport);
    commands.insert_resource(LocalNetworkId(cid));
    info!("Connect to {server_addr}");
    next_state.set(ClientState::InGame);
    commands.remove_resource::<PendingConnection>();
    Ok(())
}
