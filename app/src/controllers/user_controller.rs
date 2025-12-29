use actix_web::{HttpResponse, Responder, delete, post, web};
use mongodb::{Client, bson::doc};

use crate::{domain::{entities::mongodb::user_entity::UserEntity, objects::request::user::CreateUserRequest}, mappers::user_mapper::{from_create_user_request_to_user_entity, from_user_entity_to_create_user_response}};

#[post("")]
async fn create_user(
    mongo_client: web::Data<Client>,
    payload: web::Json<CreateUserRequest>
) -> impl Responder {
    let user_entity = from_create_user_request_to_user_entity(payload.into_inner());
    let collection = mongo_client.database("fachinis").collection::<UserEntity>("users");
    match collection.insert_one(&user_entity).await {
        Ok(_) => {
            HttpResponse::Created().json(from_user_entity_to_create_user_response(user_entity))
        }
        Err(e) => {
            println!("Error when inserting user. Error is {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/{user_id}")]
async fn delete_user(
    mongo_client: web::Data<Client>,
    path: web::Path<String>
) -> impl Responder {
    let collection = mongo_client.database("fachinis").collection::<UserEntity>("users");
    let id = path.into_inner();
    let filter = doc! {"_id": id};
    match collection.delete_one(filter).await {
        Ok(_) => {
            HttpResponse::Ok().body("OK")
        }
        Err(e) => {
            println!("Error when inserting user. Error is {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .service(create_user)
        .service(delete_user)
    );
}