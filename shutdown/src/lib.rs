#[cfg(windows)]
mod impl_win;
#[cfg(windows)]
use impl_win as impl_;
use thiserror::Error;

/// Attempt to initiate a system shutdown and poweroff.
pub fn shutdown() -> Result<(), Error> {
    impl_::shutdown()
}

#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("user does not have permission to initiate system shutdown")]
    NoPermission,
    #[error("an unknown or unhandled error occurred: .0")]
    Unknown(String),
}
