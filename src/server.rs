use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use crate::{error::CustomError, handler::*, AppState};

/// Starts the server and handles incoming requests.
pub async fn start_server() -> Result<(), CustomError> {
    // Initialize the shared state
    let app_state: AppState = Arc::new(Mutex::new(HashMap::new()));

    // Setting up router with all end points
    let router: Router = Router::new()
        .route("/", get(handle_root))
        .route("/users", post(create_user))
        .route("/users/:user_id/tasks", post(create_task).get(list_tasks))
        .route(
            "/users/:user_id/tasks/:task_id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // ---------- Start the server ---------------
    println!(">> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|err| CustomError::NotAbleToStartServer(err.to_string()))?;

    Ok(())
}
