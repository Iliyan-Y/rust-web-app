use dotenv;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, query, PgPool};
use tide::{self, Body, Request, Response, Server, StatusCode};

#[async_std::main]
async fn main() {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  // setup db
  let db_url = std::env::var("DATABASE_URL").unwrap();
  let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    .unwrap();

  // Create the server with db pool as a state
  let mut app = Server::with_state(State { db_pool });
  app.at("/").get(|req: Request<State>| async move {
    // let db_pool_from_state = &req.state().db_pool;
    // assert db is connected
    // let rows = query!("select 1 as one")
    //   .fetch_one(db_pool_from_state)
    //   .await?;
    // dbg!(rows);
    let some_json = json!([1, 2, 3, 4]);

    // let body = Body::from_json(&some_json).unwrap();
    Ok(some_json)
  });
  app.listen("localhost:3000").await.unwrap();
}

// SERVER STATE is shared across all handlers
// it need to be static
#[derive(Debug, Clone)]
struct State {
  db_pool: PgPool,
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
