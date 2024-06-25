#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Not able to start the server due to: {0}")]
    NotAbleToStartServer(String),
}
