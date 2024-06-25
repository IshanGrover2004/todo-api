use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{AppState, Task, User};

/// Handles the root("/")
pub async fn handle_root() -> Result<Json<String>, StatusCode> {
    println!(">> HANDLER - Root");

    Ok(Json("{\"Ok Working\"}".to_string()))
}

// Handler for creating a new user
pub async fn create_user(State(state): State<AppState>) -> (StatusCode, Json<Uuid>) {
    let user_id = Uuid::new_v4();
    println!("User created: {user_id}");
    let user = User {
        id: user_id,
        tasks: HashMap::new(),
        last_task_id: 0,
    };

    let mut users = state.lock().await;
    println!("At create_user state is: {:?}", state);
    users.insert(user_id, user);

    (StatusCode::CREATED, Json(user_id))
}

// Handler for creating a new task
pub async fn create_task(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(task_data): Json<Task>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    let mut users = state.lock().await;
    println!("At create_task state is: {:?}", state);
    let user = users.get_mut(&user_id).ok_or(StatusCode::NOT_FOUND)?;

    let task_id = user.last_task_id + 1;
    let task = Task {
        id: Some(task_id),
        ..task_data
    };

    user.tasks.insert(task_id, task.clone());
    user.last_task_id = task_id;

    Ok((StatusCode::CREATED, Json(task)))
}

// Handler for listing all tasks for a user
pub async fn list_tasks(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Task>>, StatusCode> {
    let users = state.lock().await;
    println!("At list_task state is: {:?}", state);
    let user = users.get(&user_id).ok_or(StatusCode::NOT_FOUND)?;

    let tasks: Vec<Task> = user.tasks.values().cloned().collect();
    Ok(Json(tasks))
}

// Handler for getting a specific task
pub async fn get_task(
    State(state): State<AppState>,
    Path((user_id, task_id)): Path<(Uuid, u32)>,
) -> Result<Json<Task>, StatusCode> {
    let users = state.lock().await;
    println!("At get_task state is: {:?}", state);
    let user = users.get(&user_id).ok_or(StatusCode::NOT_FOUND)?;
    let task = user.tasks.get(&task_id).ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task.clone()))
}

// Handler for updating a specific task
pub async fn update_task(
    State(state): State<AppState>,
    Path((user_id, task_id)): Path<(Uuid, u32)>,
    Json(updated_task): Json<Task>,
) -> Result<Json<Task>, StatusCode> {
    let mut users = state.lock().await;
    println!("At update_task state is: {:?}", state);
    let user = users.get_mut(&user_id).ok_or(StatusCode::NOT_FOUND)?;
    let task = user.tasks.get_mut(&task_id).ok_or(StatusCode::NOT_FOUND)?;

    task.title = updated_task.title;
    task.description = updated_task.description;
    task.due_date = updated_task.due_date;
    task.status = updated_task.status;

    Ok(Json(task.clone()))
}

// Handler for deleting a specific task
pub async fn delete_task(
    State(state): State<AppState>,
    Path((user_id, task_id)): Path<(Uuid, u32)>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.lock().await;
    println!("At delete_task state is: {:?}", state);
    let user = users.get_mut(&user_id).ok_or(StatusCode::NOT_FOUND)?;

    if user.tasks.remove(&task_id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
