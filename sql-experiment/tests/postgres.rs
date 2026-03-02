use sql_experiment::postgres_prelude::*;
#[tokio::test]
async fn test() -> Result<(), PostgresError> {
    let config = PostgresConfig::builder()
        .user("postgres")
        .password("postgres")
        .build();

    let _client = config.init_connection().await?;



    Ok(())
}
