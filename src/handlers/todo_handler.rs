use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Arc;

use crate::domain::models::{CreateTodoRequest, Response};
use crate::errors::errors::ApiError;
use crate::services::todo_service::TodoService;

pub async fn healtcheck() -> HttpResponse {
    let message = Response {
        message: "Everything is working fine".to_string(),
    };

    HttpResponse::Ok().json(message)
}

pub async fn handle_create_todo(
    service: web::Data<Arc<TodoService>>,
    request: web::Json<CreateTodoRequest>,
) -> Result<HttpResponse, ApiError> {
    let todo = service.create_todo(&request.into_inner().title).await?;
    Ok(HttpResponse::Created().json(todo))
}

pub async fn handle_get_todo(
    service: web::Data<Arc<TodoService>>,
    request: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let todo = service.get_todo(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(todo))
}

pub async fn handle_list_todos(
    service: web::Data<Arc<TodoService>>,
) -> Result<HttpResponse, ApiError> {
    let todos = service.list_todos().await?;
    Ok(HttpResponse::Ok().json(todos))
}

pub async fn handle_update_todo(
    service: web::Data<Arc<TodoService>>,
    request: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = request.into_inner();
    service.update_todo(id).await?;
    Ok(HttpResponse::Ok().json(Response {
        message: format!("Todo {id} updated successfully!"),
    }))
}

pub async fn handle_delete_todo(
    service: web::Data<Arc<TodoService>>,
    request: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = request.into_inner();
    service.delete_todo(id).await?;
    Ok(HttpResponse::Ok().json(Response {
        message: format!("Todo {id} deleted successfully!"),
    }))
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error": "Resource not found"
    }))
}
