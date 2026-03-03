use sql_experiment::sqlite_prelude::*;

#[tokio::test]
async fn test() -> Result<(), SqlitePoolError> {
    let config = sql_experiment::config::sqlite::SqliteConfig::builder();


    Ok(())
}
