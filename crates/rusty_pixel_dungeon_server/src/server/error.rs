use thiserror::Error;



#[derive(Debug,Error)]
pub enum InitServerError{
    #[error("get system time error")]
    GetSystemTimeError(#[from] std::time::SystemTimeError),
    #[error("net error")]
    NetError(#[from] std::io::Error)
}