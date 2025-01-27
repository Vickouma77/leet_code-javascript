use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::user::{User, CreateUser};

pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    let new_user = User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}

pub async fn get_user(user_id: web::path<Uuid>) -> HttpResponse {
    //mocking user for this use case
    let mock_user = User {
        id: *user_id,
        username: String::from("mock_user"),
        email: String::from("mock_user@test.com"),
    };

    HttpResponse::Ok().json(mock_user)
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}