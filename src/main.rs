// main.rs
use actix_web::{web, App, HttpServer};

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(db::home))
            .route("/users", web::get().to(db::get_all_users))
            .route("/users/{id}", web::get().to(db::get_user))
            .route("/users", web::post().to(db::create_user))
            .route("/users/{id}", web::put().to(db::update_user))
            .route("/users/{id}", web::delete().to(db::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
