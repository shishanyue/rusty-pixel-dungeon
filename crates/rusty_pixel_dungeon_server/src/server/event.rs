use bevy::prelude::*;
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
    V001
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomConfig{
    pub is_public:bool,
    pub max_player:u32,
    pub password:String,
    pub version:Version,
}


#[derive(Debug, Deserialize, Event, Serialize)]
pub enum RustyPixelDungeonClientNetEvent{
    NewRoom(RoomConfig)
}