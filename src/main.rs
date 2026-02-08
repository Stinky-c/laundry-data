mod db;
use crate::db::DbConfig;
use color_eyre::Report;
use color_eyre::eyre::Result;
use sql_middleware::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cfg = DbConfig::from_env()?;

    // TODO: Build pool based on `DbType` and `DbConfig`
    let pool = ConfigAndPool::new_sqlite(cfg.clone().try_into().map_err(Report::msg)?).await?; // FIXME

    let report = db::embedded::run_async(cfg).await?;
    println!("{:?}", report);

    let mut conn = pool.get_connection().await?;

    let query = QueryAndParams::new_without_params("SELECT * FROM users");
    let res = conn.query(&query.query).select().await?;
    println!("{:?}", res);

    Ok(())
}
