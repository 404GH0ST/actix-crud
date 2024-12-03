use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,

    pub title: String,

    pub completed: bool,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, message = "Title must not be empty"))]
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub message: String,
}
