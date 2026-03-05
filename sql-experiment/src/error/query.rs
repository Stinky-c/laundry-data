#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("{0}")]
    Other(String),

    #[cfg(feature = "sqlite")]
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),

}

#[cfg(feature = "sqlite")]
mod sqlite {
    use deadpool_sqlite::InteractError;
    use crate::error::query::QueryError;

    impl From<InteractError> for QueryError {
        fn from(value: InteractError) -> Self {
            match value {
                InteractError::Panic(_) => Self::Other("The sqlite callback panicked.".to_string()),
                InteractError::Aborted => Self::Other("You managed to do the impossible".to_string())
            }
        }
    }
}
