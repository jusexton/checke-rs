use thiserror::Error;

/// Error denoting an issue parsing checkers notation.
#[derive(Debug, Error)]
pub enum NotationError {
    #[error("Provided value did not conform to the expected format.")]
    InvalidFormat,

    #[error("Provided value operates outside the realm of a classical checkers board.")]
    OutOfRange,

    #[error("Provided value represented a piece standing still. The destination square was equal to the source.")]
    Idle,
}