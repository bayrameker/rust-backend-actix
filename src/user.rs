// Kullanıcıyla ilgili endpoint'ler burada tanımlanabilir.
// Örneğin, create_user, get_user, update_user, delete_user ve get_all_users fonksiyonları burada olabilir.
// Bu dosyada web modülü import edilir ve veritabanı işlemleri için db.rs import edilir.

use actix_web::{web, Responder};
use crate::db::UserDatabase;
use crate::models::User;

async fn create_user(info: web::Json<User>, db: web::Data<UserDatabase>) -> impl Responder {
    // Kullanıcı oluşturma işlemi
}

async fn get_user(path: web::Path<u64>, db: web::Data<UserDatabase>) -> impl Responder {
    // Kullanıcı getirme işlemi
}

async fn update_user(
    path: web::Path<u64>,
    info: web::Json<User>,
    db: web::Data<UserDatabase>,
) -> impl Responder {
    // Kullanıcı güncelleme işlemi
}

async fn delete_user(path: web::Path<u64>, db: web::Data<UserDatabase>) -> impl Responder {
    // Kullanıcı silme işlemi
}

async fn get_all_users(db: web::Data<UserDatabase>) -> impl Responder {
    // Tüm kullanıcıları getirme işlemi
}
