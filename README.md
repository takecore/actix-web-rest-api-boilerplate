# Actix Web boilerplate

## Setup and run server

```sh
# Need to install diesel_cli on your system to run migration
$ cargo install diesel_cli --no-default-features --features "postgres"
$ echo DATABASE_URL=postgres://username:password@yourhost/good_db_name > .env
$ make run
```

## Other

### create migration files
```
$ diesel migration generate create_xxx
```

### run migraiton
```
$ make migration
```

### rollback and run migraiton
```
$ make migration-redo
```

### reset db
```
$ make reset-db
```
