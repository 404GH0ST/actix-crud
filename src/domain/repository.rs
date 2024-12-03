use crate::errors::errors::ApiError;

use super::models::Todo;
use sqlx::PgPool;
use std::sync::Arc;

pub struct TodoRepository {
    pool: Arc<PgPool>,
}

impl TodoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn insert_todo(&self, title: &str) -> Result<Todo, ApiError> {
        let todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title, completed) VALUES ($1, false) RETURNING *",
            title
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(todo)
    }

    pub async fn find_todo(&self, id: i32) -> Result<Todo, ApiError> {
        let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound,
                _ => ApiError::DatabaseError(e),
            })?;

        Ok(todo)
    }

    pub async fn find_all_todos(&self) -> Result<Vec<Todo>, ApiError> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(todos)
    }

    pub async fn update_todo(&self, id: i32) -> Result<(), ApiError> {
        let result = sqlx::query!("UPDATE todos SET completed = true WHERE id = $1", id)
            .execute(&*self.pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    pub async fn remove_todo(&self, id: i32) -> Result<(), ApiError> {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
            .execute(&*self.pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
}
