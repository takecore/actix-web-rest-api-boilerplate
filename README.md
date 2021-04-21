# Actix-web3 REST Api boilerplate

## Setup and run server

```sh
# Need to install diesel_cli on your system to run migration.
$ cargo install diesel_cli --no-default-features --features "postgres"
$ echo DATABASE_URL=postgres://username:password@yourhost/good_db_name > .env
$ diesel setup
$ make run

# If installed cargo-watch, This can be auto recompile.
$ make watch
```

## Structure

```sh
├── Cargo.lock
├── Cargo.toml
├── diesel.toml
├── Makefile
├── migrations
│   ├── 00000000000000_diesel_initial_setup
│   │   ├── down.sql
│   │   └── up.sql
│   └── ...
├── README.md
└── src
    ├── apps
    │   ├── companies
    │   │   ├── models.rs
    │   │   ├── mod.rs
    │   │   └── views.rs
    │   ├── mod.rs
    │   ├── users
    │   │   ├── models.rs
    │   │   ├── mod.rs
    │   │   └── views.rs
    │   └── ... # create other resouce
    ├── db.rs
    ├── main.rs
    ├── schema.rs
    └── server.rs
```

## Other

### create migration files
```sh
$ diesel migration generate create_xxx
```

### run migraiton
```sh
$ make migration
```

### rollback and run migraiton
```sh
$ make migration-redo
```

### reset db
```sh
$ make reset-db
```
