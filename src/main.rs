mod config;
mod domain;
mod errors;
mod handlers;
mod routes;
mod services;

use actix_web::{web, App, HttpServer};
use anyhow::{Ok, Result};
use dotenv::dotenv;
use env_logger::Env;
use std::env;
use std::sync::Arc;

use config::database::establish_connection;
use domain::repository::TodoRepository;
use routes::todo_route::configure_routes;
use services::todo_service::TodoService;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = Arc::new(establish_connection().await?);

    let todo_repository = Arc::new(TodoRepository::new(Arc::clone(&pool)));
    let todo_service = Arc::new(TodoService::new(Arc::clone(&todo_repository)));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&todo_service)))
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
