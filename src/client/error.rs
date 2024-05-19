use bevy_replicon_renet::renet::transport::NetcodeError;
use thiserror::Error;



#[derive(Debug,Error)]
pub enum InitClientError{
    #[error("get system time error `{0}`")]
    GetSystemTimeError(#[from] std::time::SystemTimeError),
    #[error("io error `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("the netcode error `{0}`")]
    NetcodeError(#[from] NetcodeError)
}