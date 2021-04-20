Actix Web boilerplate

## Need to install diesel_cli on your system to use migration

```
$ cargo install diesel_cli --no-default-features --features "postgres"
```

put DATABASE_URL variable on .env file
```
echo DATABASE_URL=postgres://username:password@yourhost/good_db_name > .env
```

create migration files
```
$ diesel migration generate create_xxx
```

run migraiton
```
$ make migration
```

rollback and run migraiton
```
$ make migration-redo
```

reset db
```
$ make reset-db
```

## Run Server

```
make run
```