use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Window creation failed: {0}")]
    WindowCreationFailed(#[from] minifb::Error),
}
