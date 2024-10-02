use anyhow::Result;
use std::env;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let rec = sqlx::query!(
        "
        SELECT
            foo.id,
            bar.foo_id
        FROM
            foo
            LEFT JOIN bar ON bar.foo_id = foo.id
        "
    )
    .fetch_one(&pool)
    .await?;

    let _: Option<i64> = rec.foo_id;

    let rec = sqlx::query!(
        "
        SELECT
            foo.id,
            bar.foo_id
        FROM
            foo
            LEFT JOIN bar ON bar.foo_id = foo.id
        WHERE
            foo.id = $1
        ",
        5
    )
    .fetch_one(&pool)
    .await?;

    let _: i64 = rec.foo_id;

    let rec = sqlx::query!(
        "
        SELECT
            foo.id,
            bar.foo_id
        FROM
            foo
            LEFT JOIN bar ON bar.foo_id = foo.id
            AND foo.id = $1
        ",
        5
    )
    .fetch_one(&pool)
    .await?;

    let _: Option<i64> = rec.foo_id;

    Ok(())
}
