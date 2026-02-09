use color_eyre::{Report, eyre::Result};
use refinery::config::{Config, ConfigDbType};
use sql_middleware::DatabaseType;
use std::env::var;
use std::fmt::{Debug, Formatter};
use std::result::Result as StdResult;
use std::str::FromStr;

#[cfg(not(any(
    feature = "postgres",
    feature = "sqlite",
    feature = "turso",
    feature = "mssql"
)))]
compile_error!("Requires at least one db driver.");

pub(crate) mod embedded {
    use crate::db::{DbConfig, DbType};
    use color_eyre::{Report, eyre, eyre::Result};
    use refinery::{Report as RunnerReport, config::Config};

    // Dynamically compile in migration for each driver

    #[cfg(feature = "postgres")]
    pub(crate) mod postgres {
        refinery::embed_migrations!("migrations/postgres");
    }

    #[cfg(any(feature = "sqlite", feature = "turso"))]
    pub(crate) mod sqlite {
        refinery::embed_migrations!("migrations/sqlite");
    }

    #[cfg(feature = "mssql")]
    pub(crate) mod mssql {
        suprefineryer::embed_migrations!("migrations/mssql");
    }

    pub(crate) async fn run_async(db_config: DbConfig) -> Result<RunnerReport> {
        let db_type = db_config.db_type;
        let mut config: Config = db_config.try_into().map_err(Report::msg)?;

        match db_type {
            // Migrations are not compiled in if feature disabled
            #[cfg(feature = "postgres")]
            DbType::Postgres => {
                let runner = postgres::migrations::runner();
                Ok(runner.run_async(&mut config).await?)
            }
            #[cfg(any(feature = "sqlite", feature = "turso"))]
            DbType::Sqlite | DbType::Turso => {
                let runner = sqlite::migrations::runner();
                Ok(runner.run(&mut config)?)
            }
            #[cfg(feature = "mssql")]
            DbType::Mssql => {
                let runner = mssql::migrations::runner();
                Ok(runner.run_async(&mut config).await?)
            }

            _ => Err(eyre::eyre!("Unsupported database type {:?}", db_type)),
        }
    }
}

/// Type of Database. One of `postgres`, `pgsql`, `sqlite`, `mssql`, `turso`.
static ENV_DB_TYPE: &str = "DB_TYPE";
/// Hostname or ip to connect to.
static ENV_DB_HOST: &str = "DB_HOST";
/// Port to connect to.
static ENV_DB_PORT: &str = "DB_PORT";
/// Database to connect to.
static ENV_DB_NAME: &str = "DB_NAME";
/// Database Username.
static ENV_DB_USER: &str = "DB_USER";
/// Database password.
static ENV_DB_PASS: &str = "DB_PASS";
/// Database file path. Used for sqlite.
static ENV_DB_PATH: &str = "DB_PATH";

#[derive(Clone)]
pub(crate) struct DbConfig {
    pub(crate) db_type: DbType,
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) db_name: Option<String>,
    pub(crate) user_name: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) path: Option<String>,
}

// Hide password from debug
impl Debug for DbConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbConfig")
            .field("db_type", &self.db_type)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("db_name", &self.db_name)
            .field("username", &self.user_name)
            .field("path", &self.path)
            .finish()
    }
}

impl DbConfig {
    pub(crate) fn from_env() -> Result<Self> {
        let db_type = var(ENV_DB_TYPE)?.parse::<DbType>().map_err(Report::msg)?;

        let port: Option<u16> = match var(ENV_DB_PORT) {
            Ok(value) => match value.as_str() {
                "" => None,
                v if !v.is_empty() => Some(value.parse().map_err(Report::msg)?),
                _ => None, // Should be unreachable, but just in case
            },
            Err(_) => None,
        };

        Ok(Self {
            db_type,
            host: var(ENV_DB_HOST).ok(),
            port,
            db_name: var(ENV_DB_NAME).ok(),
            user_name: var(ENV_DB_USER).ok(),
            password: var(ENV_DB_PASS).ok(),
            path: var(ENV_DB_PATH).ok(),
        })
    }
}

#[cfg(feature = "sqlite")]
impl TryFrom<DbConfig> for sql_middleware::SqliteOptions {
    type Error = String;

    fn try_from(config: DbConfig) -> StdResult<Self, Self::Error> {
        if config.db_type != DbType::Sqlite {
            return Err("DbType is not sqlite".to_string());
        }

        Ok(Self::new(config.path.ok_or("Could not find path")?))
    }
}

#[cfg(feature = "postgres")]
impl TryFrom<DbConfig> for sql_middleware::PgConfig {
    type Error = String;

    fn try_from(config: DbConfig) -> StdResult<Self, Self::Error> {
        if config.db_type != DbType::Postgres {
            return Err("DbType is not postgres".to_string());
        }
        let mut pg = sql_middleware::PgConfig::new();
        pg.host = config.host;
        pg.port = config.port;
        pg.dbname = config.db_name;
        pg.user = config.user_name;
        pg.password = config.password;
        Ok(pg)
    }
}

#[cfg(feature = "postgres")]
impl TryFrom<DbConfig> for sql_middleware::PostgresOptions {
    type Error = String;
    fn try_from(config: DbConfig) -> StdResult<Self, Self::Error> {
        if config.db_type != DbType::Postgres {
            return Err("DbType is not postgres".to_string());
        }
        let pg = config.try_into()?;

        Ok(Self::new(pg))
    }
}

#[cfg(feature = "mssql")]
impl TryFrom<DbConfig> for sql_middleware::MssqlOptions {
    type Error = String;
    fn try_from(config: DbConfig) -> StdResult<Self, Self::Error> {
        if config.db_type != DbType::Mssql {
            return Err("DbType is not Mssql".to_string());
        }
        Ok(Self::new(
            config.host.ok_or("Missing host")?,
            config.db_name.ok_or("Missing db_name")?,
            config.user_name.ok_or("Missing username")?,
            config.password.ok_or("Missing Password")?,
            config.port,
            None,
        ))
    }
}

impl TryFrom<DbConfig> for Config {
    type Error = String;
    fn try_from(value: DbConfig) -> StdResult<Self, Self::Error> {
        match value.db_type {

            #[cfg(feature = "postgres")]
            DbType::Postgres => {
                unimplemented!() // TODO: Complete impl
            }
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => {
                Ok(Config::new(ConfigDbType::Sqlite)
                    .set_db_path(&value.path.ok_or("Missing path")?))
            }

            #[cfg(feature = "mssql")]
            DbType::Mssql => {
                unimplemented!() // TODO: Complete impl
            }
            #[cfg(feature = "turso")]
            DbType::Turso => {
                unimplemented!() // TODO: Complete impl
            }
            _ => Err("Missing driver support".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub(crate) enum DbType {
    Postgres,
    Sqlite,
    Mssql,
    Turso,
}

impl FromStr for DbType {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "postgres" | "pgsql " => Ok(Self::Postgres),
            "mssql" => Ok(Self::Mssql),
            "sqlite" => Ok(Self::Sqlite),
            "turso" => Ok(Self::Turso),
            _ => Err(format!("Unknown database type: {}", value)),
        }
    }
}

impl From<DbType> for DatabaseType {
    fn from(value: DbType) -> Self {
        match value {
            #[cfg(feature = "postgres")]
            DbType::Postgres => Self::Postgres,
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => Self::Sqlite,
            #[cfg(feature = "turso")]
            DbType::Turso => Self::Sqlite,
            #[cfg(feature = "mssql")]
            DbType::Mssql => Self::Mssql,
            _ => unimplemented!(),
        }
    }
}
impl From<DbType> for ConfigDbType {
    fn from(value: DbType) -> Self {
        match value {
            DbType::Postgres => Self::Postgres,
            DbType::Sqlite => Self::Sqlite,
            DbType::Mssql => Self::Mssql,
            DbType::Turso => Self::Sqlite,
        }
    }
}
