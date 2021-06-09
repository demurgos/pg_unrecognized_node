use sqlx::{Connection, PgConnection};
use sqlx::postgres::PgConnectOptions;
use std::error::Error;

#[tokio::main]
async fn main() {
  let opt = PgConnectOptions::new()
    .host("localhost")
    .port(5432)
    .database("etwin.dev")
    .username("etwin.dev.admin")
    .password("dev");
  let mut con = PgConnection::connect_with(&opt).await.unwrap();
  migrate(&mut con).await.unwrap();
  con.close().await.unwrap();
}

async fn migrate(con: &mut PgConnection) -> Result<(), Box<dyn Error>> {
  // Note: removing _any_ of the `CHECK ...` clauses fixes the issue.
  // It may imply that the issue seems related to having to parsing the `CHECK`
  // twice during the same session.
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
    sqlx::query(dbg!(&query)).execute(&mut *con).await?;
  }

  Ok(())
}
