pub use bevy::prelude::*;
use bevy_replicon::prelude::*;
use server::{event::RustyPixelDungeonClientNetEvent, RustyPixelDungeonServer};

pub mod server;

pub struct RustyPixelDungeonServerPlugin;

impl Plugin for RustyPixelDungeonServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_client_event::<RustyPixelDungeonClientNetEvent>(ChannelKind::Ordered)
        .init_resource::<RustyPixelDungeonServer>()
        .add_systems(Update, new_con.run_if(server_running))
        .add_systems(Update,receive_events);
    }
}

fn new_con(mut server_events: EventReader<ServerEvent>){
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("{client_id:?} connected");
                // Generate pseudo random color from client id.
                
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("{client_id:?} disconnected: {reason}");
            }
        }
    }
}

fn receive_events(mut clinet_events: EventReader<FromClient<RustyPixelDungeonClientNetEvent>>) {
    for FromClient { client_id, event } in clinet_events.read() {
        info!("received event {event:?} from {client_id:?}");
    }
}