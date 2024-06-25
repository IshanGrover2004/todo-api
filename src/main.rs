use todo_api::{error::CustomError, server::start_server};

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    start_server().await?;

    Ok(())
}
