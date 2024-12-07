use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),

    #[error(transparent)]
    HandlebarsTemplate(#[from] handlebars::TemplateError),
    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    AxumHttp(#[from] axum::http::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{self}").into_response()
    }
}
