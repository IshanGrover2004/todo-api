use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

pub mod error;
pub mod handler;
pub mod server;
#[cfg(test)]
pub mod test;

pub type AppState = Arc<Mutex<HashMap<Uuid, User>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Uuid,
    tasks: HashMap<u32, Task>,
    last_task_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    id: Option<u32>,
    title: String,
    description: String,
    due_date: DateTime<Utc>,
    status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}
