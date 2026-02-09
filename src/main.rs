mod db;
use crate::db::DbConfig;
use color_eyre::eyre::Result;
use sql_middleware::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cfg = DbConfig::from_env()?;

    let pool = db::new_pool(cfg.clone()).await?;

    let report = db::embedded::run_async(cfg).await?;
    println!("{:?}", report);

    let mut conn = pool.get_connection().await?;

    let query = QueryAndParams::new_without_params("SELECT * FROM mssql");
    let res = conn.query(&query.query).select().await?;
    println!("{:?}", res);

    Ok(())
}
