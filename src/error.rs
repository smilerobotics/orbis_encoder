use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("orbis: Failed to open: path({:?}) Error({:?})", path, source)]
    AsyncSerialFailedToOpen {
        #[source]
        source: serialport::Error,
        path: PathBuf,
    },

    #[error("orbis: Failed to send: Error({:?})", .0)]
    AsyncSerialFailedToSend(std::io::Error),

    #[error("orbis: Failed to receive: Error({:?})", .0)]
    AsyncSerialFailedToReceive(std::io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
