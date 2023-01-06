use dotenv;
use serde::Serialize;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool, Pool, Postgres};
use tide::{self, Body, Request, Response, Server, StatusCode};
use uuid::Uuid;

#[async_std::main]
async fn main() {
  let app = server().await;
  app.listen("localhost:3000").await.unwrap();
}

async fn make_db_pool() -> Pool<Postgres> {
  let db_url = std::env::var("DATABASE_URL").unwrap();

  PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    .unwrap()
}

async fn server() -> Server<State> {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  let db_pool = make_db_pool().await;

  // Create the server with db pool as a state
  let mut app = Server::with_state(State { db_pool });

  app.at("/").get(|req: Request<State>| async move {
    let db_pool_from_state = &req.state().db_pool;
    // assert db is connected
    let users = query_as!(User, "select id, username from users")
      .fetch_all(db_pool_from_state)
      .await?;
    dbg!(users);
    let some_json = json!([1, 2, 3, 4]);
    //let body = Body::from_json(&users)?;
    Ok(some_json)

    // Ok(Response::new(StatusCode::Ok).set_body(users))
  });
  app
}

// SERVER STATE is shared across all handlers
// it need to be static
#[derive(Debug, Clone)]
struct State {
  db_pool: PgPool,
}

#[derive(Debug, Serialize)]
struct User {
  id: Uuid,
  username: String,
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
