use dotenv;
use sqlx::{postgres::PgPoolOptions, query};
use tide;

#[async_std::main]
async fn main() -> Result<(), Error> {
  dotenv::dotenv().ok();
  pretty_env_logger::init();
  let mut app = tide::new();

  // setup db
  let db_url = std::env::var("DATABASE_URL")?;
  let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await?;
  // assert db is connected
  let rows = query!("select 1 as one").fetch_one(&db_pool).await?;
  dbg!(rows);

  app.at("/").get(|_| async move { Ok("HELLO WORLD") });
  app.listen("localhost:3000").await?;

  Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
  #[error(transparent)]
  DbError(#[from] sqlx::Error),

  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error(transparent)]
  VarError(#[from] std::env::VarError),
}
