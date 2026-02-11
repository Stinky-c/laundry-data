# Architecture

## Project Setup

Primary tooling is
- [mise](https://mise.jdx.com)
- [Docker](https://docs.docker.com/engine/) + [Docker Compose](https://docs.docker.com/compose/)

Mise is for installing tools and managing run environments for all features.
Docker runs mssql and postgres containers as needed for the program.

To start, set `MISE_ENV` to any of `sqlite`, `postgres`, `mssql`. Then run `db:up`, 
then `app:run` for debug.

### Tasks
All mise tasks for each environment.

#### `sqlite`
```console
Name         Description

<!-- cmdrun mise tasks ls --env sqlite --local --raw -->
```

#### `postgres`
```console
Name         Description

<!-- cmdrun mise tasks ls --env postgres --local --raw -->
```

#### `mssql`
```console
Name         Description

<!-- cmdrun mise tasks ls --env mssql --local --raw -->
```


### Troubleshooting

#### `mssql`
If mssql server fails to start or read/write to the data directory, set `COMPOSE_UID` 
and `COMPOSE_GID` to your user and group.

Example `mise.{ENV}.local.toml` or `mise.local.toml`. These files are ignored by git.
```toml
[env]
COMPOSE_UID = "1000"
COMPOSE_GID = "1000"
```