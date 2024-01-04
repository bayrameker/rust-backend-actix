// db.rs
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

pub async fn home() -> HttpResponse {
    
    HttpResponse::Ok().body("Sayfaya hoş geldin.")
}

pub async fn get_all_users() -> HttpResponse {
    // Burada tüm kullanıcıları getirecek kodu yazabilirsiniz.
    HttpResponse::Ok().body("Tüm kullanıcılar listelendi.")
}

pub async fn get_user() -> HttpResponse {
    // Burada belirli bir kullanıcıyı getirecek kodu yazabilirsiniz.
    HttpResponse::Ok().body("Belirli bir kullanıcı getirildi.")
}

pub async fn create_user(_user: web::Json<User>) -> HttpResponse {
    // Değişken kullanılmadığı için alt çizgiyle ifade edildi.
    HttpResponse::Ok().body("Kullanıcı oluşturuldu.")
}

pub async fn update_user() -> HttpResponse {
    // Burada bir kullanıcıyı güncelleyecek kodu yazabilirsiniz.
    HttpResponse::Ok().body("Kullanıcı güncellendi.")
}

pub async fn delete_user() -> HttpResponse {
    // Burada bir kullanıcıyı silecek kodu yazabilirsiniz.
    HttpResponse::Ok().body("Kullanıcı silindi.")
}
