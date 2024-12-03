use actix_web::{middleware::Logger, web};

use crate::handlers::todo_handler::{
    handle_create_todo, handle_delete_todo, handle_get_todo, handle_list_todos, handle_update_todo,
    healtcheck, not_found,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthcheck", web::get().to(healtcheck))
            .route("/todos", web::post().to(handle_create_todo))
            .route("/todos", web::get().to(handle_list_todos))
            .route("/todos/{id}", web::get().to(handle_get_todo))
            .route("/todos/{id}", web::put().to(handle_update_todo))
            .route("/todos/{id}", web::delete().to(handle_delete_todo))
            .default_service(web::route().to(not_found))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i")),
    );
}
