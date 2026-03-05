use sql_experiment::error::common::CommonError;
use sql_experiment::pool::common::ToConnection;
use sql_experiment::sqlite_prelude::*;

#[tokio::test]
async fn test() -> Result<(), CommonError> {
    let config = SqliteConfig::builder().path(":memory:").build();
    let pool = config.to_pool()?;
    let _conn = pool.get_connection().await?;




    Ok(())
}
