use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use log::error;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Todo not found")]
    NotFound,

    #[error("Invalid input")]
    ValidationError,

    #[error("Database error")]
    DatabaseError(#[source] sqlx::Error),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ValidationError => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(json!({
                    "error": "Todo not found"
                })),
            ApiError::ValidationError => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(json!({
                    "error": "Invalid input"
                })),
            ApiError::DatabaseError(err) => {
                error!("Database error: {}", err);
                HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .json(json!({
                        "error": "Internal server error"
                    }
                    ))
            }
            ApiError::InternalError => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(json!({
                    "error": "Internal server error"
                }
                )),
        }
    }
}
