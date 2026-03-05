use sql_experiment::error::common::CommonError;
use sql_experiment::mssql_prelude::*;
use sql_experiment::pool::common::ToConnection;

#[tokio::test]
async fn test() -> Result<(), CommonError> {
    let config = MssqlConfig::builder()
        .auth_method(AuthMethod::sql_server("sa", "Thisisapassword123!"))
        .build();

    let pool = config.to_pool()?;
    let _conn = pool.get_connection().await?;

    Ok(())
}
