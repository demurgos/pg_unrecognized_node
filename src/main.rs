use sqlx::{PgPool, Transaction, Postgres};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::error::Error;

#[tokio::main]
async fn main() {
  let pool: PgPool = PgPoolOptions::new()
    .max_connections(5)
    .connect_with(
      PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("etwin.dev")
        .username("etwin.dev.admin")
        .password("dev"),
    )
    .await
    .unwrap();
  let mut tx = pool.begin().await.unwrap();
  migrate(&mut tx).await.unwrap();
  tx.commit().await.unwrap();
  pool.close().await;
}

async fn migrate(tx: &mut Transaction<'_, Postgres>) -> Result<(), Box<dyn Error>> {
  let queries = [
    "DROP DOMAIN IF EXISTS schema_meta;",
    "DROP TYPE IF EXISTS raw_schema_meta;",
    "CREATE TYPE raw_schema_meta AS (version int4);",
    "CREATE DOMAIN schema_meta AS raw_schema_meta CHECK ((value).version IS NOT NULL AND (value).version >= 1);",
    "DROP DOMAIN IF EXISTS schema_meta;",
    // The request below fails with the message `"unrecognized node type: X"`
    "CREATE DOMAIN schema_meta AS raw_schema_meta CHECK ((value).version IS NOT NULL AND (value).version >= 1);",
  ];

  for query in std::array::IntoIter::new(queries) {
    sqlx::query(dbg!(&query)).execute(&mut *tx).await?;
  }

  Ok(())
}
