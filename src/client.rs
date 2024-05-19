pub mod error;
pub mod plugin;

use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

pub use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, RenetClient, RenetServer
    },
    RenetChannelsExt,
};

use self::error::InitClientError;

const PROTOCOL_ID: u64 = 0;

pub enum RustyPixelDungeonClientEvent {
    Inited,
}
#[derive(Resource, Default)]
pub struct RustyPixelDungeonClient;

impl RustyPixelDungeonClient {
    pub fn init_client(
        &self,
        commands: &mut Commands,
        channels: &Res<RepliconChannels>,
    ) -> Result<(), InitClientError> {
        let server_channels_config = channels.get_server_configs();
        let client_channels_config = channels.get_client_configs();

        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let client_id = current_time.as_millis() as u64;
        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5123);
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

        commands.insert_resource(client);
        commands.insert_resource(transport);

        Ok(())
    }
}
