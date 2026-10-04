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

```console
Name         Description

<!-- cmdrun mise tasks ls --env postgres --local --raw -->
```

### Dependencies

```console
<!-- cmdrun mise run --env postgres app:deps  -->
```

### Troubleshooting

