use thiserror::Error;

use super::RoomSize;



#[derive(Debug,Error)]
pub enum RoomError{
    #[error("this room no such size")]
    NoSuchSize(RoomSize)
}