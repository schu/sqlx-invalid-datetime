
use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use sqlx::types::chrono::{DateTime, Utc};

#[tokio::main]

async fn main() -> Result<()> {
    let db = SqlitePool::connect("test.sqlite3").await?;

    sqlx::query!(
        r#"
        INSERT INTO foo (ts) VALUES ('Mon, 12 Aug 2024 12:00:00 +0000');
        "#,
    )
    .execute(&db)
    .await?;

    let result = sqlx::query!(
        r#"
        SELECT ts as "ts: String"
        FROM foo
        "#,
    )
    .fetch_one(&db)
    .await?;

    println!("{:#?}", result);

    let result = sqlx::query!(
        r#"
        SELECT ts
        FROM foo
        "#,
    )
    .fetch_one(&db)
    .await?;

    println!("{:#?}", result);

    // let result = sqlx::query!(
    //     r#"
    //     SELECT ts as "ts: DateTime<Utc>"
    //     FROM foo
    //     "#,
    // )
    // .fetch_one(&db)
    // .await?;

    // println!("{:#?}", result);

    Ok(())
}