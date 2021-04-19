Actix Web boilerplate

## Need to install diesel_cli on your system to use migration

```
$ cargo install diesel_cli --no-default-features --features "mysql"
```

put DATABASE_URL variable on .env file
```
echo DATABASE_URL=mysql://username:password@yourhost/good_db_name > .env
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

## Run

```
make run
```