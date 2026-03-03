use sql_experiment::mssql_prelude::*;
#[tokio::test]
async fn test() -> Result<(), MsSqlError> {
    let config = MssqlConfig::builder()
        .auth_method(AuthMethod::sql_server("sa", "Thisisapassword123!"))
        .build();

    Ok(())
}
