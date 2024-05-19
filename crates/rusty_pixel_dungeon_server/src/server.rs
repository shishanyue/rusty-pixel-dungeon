pub mod error;
pub mod event;
use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

pub use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    RenetChannelsExt,
};
use crate::server::error::InitServerError;
const PROTOCOL_ID: u64 = 0;

pub enum RustyPixelDungeonServerEvent {
    Inited,
}
#[derive(Resource, Default)]
pub struct RustyPixelDungeonServer;

impl RustyPixelDungeonServer {
    pub fn init_server(
        &self,
        commands: &mut Commands,
        channels: &Res<RepliconChannels>,
    ) -> Result<(), InitServerError> {
        let server_channels_config = channels.get_client_configs();
        let client_channels_config = channels.get_client_configs();

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5123);
        let socket = UdpSocket::bind(public_addr)?;
        let server_config = ServerConfig {
            current_time,
            max_clients: 10,
            protocol_id: PROTOCOL_ID,
            authentication: ServerAuthentication::Unsecure,
            public_addresses: vec![public_addr],
        };
        let transport = NetcodeServerTransport::new(server_config, socket)?;

        commands.insert_resource(server);
        commands.insert_resource(transport);

        Ok(())
    }
}
