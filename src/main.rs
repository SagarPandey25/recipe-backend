mod api_response;
mod entities;
mod service;

use crate::{api_response::ApiResponse, entities::recipe, service::DBQuery};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::env;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Deserialize, Debug)]
pub struct GetRecipiesQueryParams {
    pub query: Option<String>,
}

async fn list_recipies(
    state: State<AppState>,
    Query(params): Query<GetRecipiesQueryParams>,
) -> Json<ApiResponse<Vec<recipe::Model>>> {
    match DBQuery::find_recipies(&state.conn, params.query).await {
        Ok(recipies) => Json(ApiResponse::success(recipies)),
        Err(err) => Json(ApiResponse::error(format!("Cannot find recipies: {}", err))),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn start() -> Result<(), std::io::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let mut connect_options = ConnectOptions::new(db_url);
    connect_options.sqlx_logging(true);

    let conn = Database::connect(connect_options)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let app = Router::new()
        .route("/", get(list_recipies))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
