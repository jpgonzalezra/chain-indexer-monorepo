# chain-indexer

A modular indexer system for EVM compatibles

### run migration

1 - if the database does not exist

```bash
sqlx database create --database-url postgresql://{username}:${password}@{host}:{host}/{database}
```

2 - run migrations

```bash
$ sqlx migrate run --database-url postgresql://{username}:${password}@{host}:{host}/{database}
```
