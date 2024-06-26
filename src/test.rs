#[cfg(test)]
mod tests {
    use crate::*;

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    use chrono::Utc;
    use handler::*;

    #[tokio::test]
    async fn test_create_user() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (status, Json(user_id)) = create_user(State(state.clone())).await;

        assert_eq!(status, StatusCode::CREATED);

        let users = state.lock().await;
        assert!(users.contains_key(&user_id));
    }

    #[tokio::test]
    async fn test_create_task() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (_, Json(user_id)) = create_user(State(state.clone())).await;

        let task_data = Task {
            id: Some(1),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::Todo,
        };

        let result =
            create_task(State(state.clone()), Path(user_id), Json(task_data.clone())).await;

        assert!(result.is_ok());
        let (status, Json(created_task)) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created_task.title, task_data.title);
        assert_eq!(created_task.description, task_data.description);
        assert_eq!(created_task.status, task_data.status);
    }

    #[tokio::test]
    async fn test_list_tasks() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (_, Json(user_id)) = create_user(State(state.clone())).await;

        let task_data = Task {
            id: Some(1),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::Todo,
        };

        let _ = create_task(State(state.clone()), Path(user_id), Json(task_data.clone()))
            .await
            .unwrap();

        let result = list_tasks(State(state.clone()), Path(user_id)).await;

        assert!(result.is_ok());
        let Json(tasks) = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, task_data.title);
    }

    #[tokio::test]
    async fn test_get_task() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (_, Json(user_id)) = create_user(State(state.clone())).await;

        let task_data = Task {
            id: Some(1),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::Todo,
        };

        let (_, Json(created_task)) =
            create_task(State(state.clone()), Path(user_id), Json(task_data.clone()))
                .await
                .unwrap();

        let result = get_task(
            State(state.clone()),
            Path((user_id, created_task.id.unwrap())),
        )
        .await;

        assert!(result.is_ok());
        let Json(retrieved_task) = result.unwrap();
        assert_eq!(retrieved_task.id, created_task.id);
        assert_eq!(retrieved_task.title, task_data.title);
    }

    #[tokio::test]
    async fn test_update_task() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (_, Json(user_id)) = create_user(State(state.clone())).await;

        let task_data = Task {
            id: Some(1),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::Todo,
        };

        let (_, Json(created_task)) =
            create_task(State(state.clone()), Path(user_id), Json(task_data.clone()))
                .await
                .unwrap();

        let updated_task_data = Task {
            id: created_task.id,
            title: "Updated Task".to_string(),
            description: "This is an updated test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::InProgress,
        };

        let result = update_task(
            State(state.clone()),
            Path((user_id, created_task.id.unwrap())),
            Json(updated_task_data.clone()),
        )
        .await;

        assert!(result.is_ok());
        let Json(updated_task) = result.unwrap();
        assert_eq!(updated_task.id, created_task.id);
        assert_eq!(updated_task.title, updated_task_data.title);
        assert_eq!(updated_task.description, updated_task_data.description);
        assert_eq!(updated_task.status, updated_task_data.status);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let state: AppState = Arc::new(Mutex::new(HashMap::new()));
        let (_, Json(user_id)) = create_user(State(state.clone())).await;

        let task_data = Task {
            id: Some(1),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            due_date: Utc::now(),
            status: TaskStatus::Todo,
        };

        let (_, Json(created_task)) =
            create_task(State(state.clone()), Path(user_id), Json(task_data.clone()))
                .await
                .unwrap();

        let result = delete_task(
            State(state.clone()),
            Path((user_id, created_task.id.unwrap())),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);

        let users = state.lock().await;
        let user = users.get(&user_id).unwrap();
        assert!(!user.tasks.contains_key(&created_task.id.unwrap()));
    }
}
