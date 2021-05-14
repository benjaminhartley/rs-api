# Rs-api

## Setup

### Dependencies

#### Postgres

#### Rust / Cargo

#### Migrant

```bash
cargo install migrant
```

### Database

#### Initialize

```bash
make initdb
```

#### Update with latest migrations

```bash
make updatedb
```

#### Drop

```bash
make dropdb
```

## Test

```bash
make test
```

### Environment

Environment variables should be stored in the .env file for debug and the .env_prod file for prod.  Check .env.example to see which values are required.

#### secret_key

Generate a new secret key for production:

```bash
openssl rand -base64 32
```

## Run

### Debug

Running debug will remove all previous log files on startup

```bash
make run_debug
```

### Release

Running production will archive all previous log files on startup

```bash
make run
```
