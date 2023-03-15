use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, get, put, delete, 
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId; 

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    //create_user code goes here
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //get_user code goes here
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    //update_user code goes here
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //delet_user code goes here
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}