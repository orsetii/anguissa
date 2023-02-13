use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;




#[derive(Error, Debug)]
pub enum Error {
    // TODO this is just an example error.
    #[error("Invalid Frame")]
    InvalidFrame,
    #[error("{0}")]
    Unknown(String),
}