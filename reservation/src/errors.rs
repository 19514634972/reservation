use std::io;
use thiserror::Error;


#[derive(Error,Debug)]
pub enum ReservationError{
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("tht data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?},fun {found:?})")]
    // InvalidHeader {expected:String,found:String},
    #[error("unkown data store error")]
    Unknown,
}