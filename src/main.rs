mod db;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = db::DbPool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::post().to(handlers::create_todo))
            .route("/todos", web::get().to(handlers::list_todos))
            .route("/todos/{id}", web::put().to(handlers::update_todo))
            .route("/todos/{id}", web::delete().to(handlers::delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}