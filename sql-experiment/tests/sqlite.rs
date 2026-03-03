use sql_experiment::sqlite_prelude::*;

#[tokio::test]
async fn test() -> Result<(), SqlitePoolError> {
    let config = SqliteConfig::builder().path(":memory:").build();


    Ok(())
}
