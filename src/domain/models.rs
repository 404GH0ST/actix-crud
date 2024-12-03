use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,

    pub title: String,

    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub message: String,
}
