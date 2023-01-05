use dotenv;
use tide;

#[async_std::main]
async fn main() -> Result<(), Error> {
  dotenv::dotenv().ok();
  let mut app = tide::new();
  let db_url = std::env::var("DATABASE_URL").unwrap();

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
}
