use std::env;

use euclid::{app::App, error::Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let router = App::new().await?.into_router();

    let server_url = env::var("SERVER_URL")?;
    let listener = TcpListener::bind(&server_url).await?;
    println!("Listening on: {server_url}");

    axum::serve(listener, router).await?;
    Ok(())
}
