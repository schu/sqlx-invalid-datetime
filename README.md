```
export DATABASE_URL=sqlite://test.sqlite3
cargo sqlx database create
cargo sqlx migrate run
cargo run
```
