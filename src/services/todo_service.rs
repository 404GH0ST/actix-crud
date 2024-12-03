use std::sync::Arc;
use validator::Validate;

use crate::{
    domain::{
        models::{CreateTodoRequest, Todo},
        repository::TodoRepository,
    },
    errors::errors::ApiError,
};

pub struct TodoService {
    repository: Arc<TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<TodoRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_todo(&self, request: CreateTodoRequest) -> Result<Todo, ApiError> {
        request.validate().map_err(|_| ApiError::ValidationError)?;
        let todo = self.repository.insert_todo(&request.title).await?;
        Ok(todo)
    }

    pub async fn get_todo(&self, id: i32) -> Result<Todo, ApiError> {
        if id <= 0 {
            return Err(ApiError::NotFound);
        }

        let todo = self.repository.find_todo(id).await?;
        Ok(todo)
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>, ApiError> {
        let todos = self.repository.find_all_todos().await?;
        Ok(todos)
    }

    pub async fn update_todo(&self, id: i32) -> Result<(), ApiError> {
        if id <= 0 {
            return Err(ApiError::NotFound);
        }
        self.repository.update_todo(id).await?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), ApiError> {
        if id <= 0 {
            return Err(ApiError::NotFound);
        }
        self.repository.remove_todo(id).await?;
        Ok(())
    }
}
