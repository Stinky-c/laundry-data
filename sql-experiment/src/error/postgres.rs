#[derive(thiserror::Error, Debug)]
pub enum PostgresError {
    #[error(transparent)]
    ServerCommunicationError(#[from] tokio_postgres::Error),
    #[error(transparent)]
    PostgresError(#[from] tokio_postgres::error::DbError),
}
