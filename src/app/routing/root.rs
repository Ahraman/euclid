use axum::response::{IntoResponse, Redirect};

pub async fn get() -> impl IntoResponse {
    Redirect::to("/w/main")
}
