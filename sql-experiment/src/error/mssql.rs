
#[derive(thiserror::Error, Debug)]
pub enum MsSqlError {
    #[error(transparent)]
    TiberiusError(#[from] tiberius::error::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
