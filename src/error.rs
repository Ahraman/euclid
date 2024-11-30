#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
}
