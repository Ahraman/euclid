use std::env;

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use euclid::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let router = Router::new().route("/", get(root));

    let server_url = env::var("SERVER_URL")?;
    let listener = TcpListener::bind(&server_url).await?;
    println!("Listening on: {server_url}");

    axum::serve(listener, router).await?;
    Ok(())
}

async fn root() -> Response {
    "Hello world!".into_response()
}
