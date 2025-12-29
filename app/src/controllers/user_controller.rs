use std::fmt::format;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web::{self, Json}};
use deadpool_redis::Pool;
use redis::AsyncCommands;
use log::{ info, error };

use crate::{db::repositories::mongodb::user_repository::UserRepository, domain::{entities::mongodb::user_entity::UserEntity, exceptions::db::DatabaseExceptionType, objects::{request::user::{CreateUserRequest, UpdateUserRequest}, response::{common::SampleMessage, user::{UpdateUserResponse, UserResponse}}}}, mappers::user_mapper::{from_create_user_request_to_user_entity, from_user_entity_to_create_user_response, from_user_entity_to_user_response}};

#[post("")]
async fn create_user(
    user_repository: web::Data<UserRepository>,
    redis_connection_pool: web::Data<Pool>,
    payload: web::Json<CreateUserRequest>
) -> impl Responder {
    let user_entity = from_create_user_request_to_user_entity(payload.into_inner());
    match user_repository.insert_user(user_entity).await {
        Ok(new_user_entity) => {
            if let Ok(mut conn) = redis_connection_pool.get().await {
                if let Ok(serialized_object) = serde_json::to_string(&new_user_entity) {
                    match conn.set_ex::<_, _, ()>(&new_user_entity.username, serialized_object, 60).await {
                        Ok(_) => {
                            info!("User {} updated on redis db", &new_user_entity.username);
                        }
                        Err(e) => {
                            error!("Error when updating redis cluster: {}", e);
                        }
                    }
                }
            }
            HttpResponse::Created().json(from_user_entity_to_create_user_response(new_user_entity))
        }
        Err(database_exception) => {
            if database_exception.kind == DatabaseExceptionType::EntityAlreadyExists {
                HttpResponse::UnprocessableEntity().json(SampleMessage{ message: "User already exists!".to_string()})
            } else {
                error!("Error when inserting user. Error is {}", database_exception.message);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[delete("/{user_id}")]
async fn delete_user(
    user_repository: web::Data<UserRepository>,
    path: web::Path<String>
) -> impl Responder {
    let id = path.into_inner();
    match user_repository.delete_by_id(&id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().json(SampleMessage{ message: format(format_args!("User {} deleted!", &id.to_string())) })
            } else {
                HttpResponse::Ok().json(SampleMessage{ message: format(format_args!("User {} does not exist!", &id.to_string())) })
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{user_id}")]
async fn get_user(
    user_repository: web::Data<UserRepository>,
    redis_connection_pool: web::Data<Pool>,
    path: web::Path<String>
) -> impl Responder {
    let id = path.into_inner();

    if let Ok(mut conn) = redis_connection_pool.get().await {
        // We specify the type for redis_result as Option<String>
        // because the key might not exist in Redis.
        match conn.get::<_, Option<String>>(&id).await {
            Ok(Some(user_json)) => {
                info!("Redis Cache Hit! {}", user_json);
                if let Ok(user_entity) = serde_json::from_str::<UserEntity>(&user_json) {
                    info!("returning data from redis cache");
                    return HttpResponse::Ok().json(from_user_entity_to_user_response(user_entity));
                }
            }
            Ok(None) => info!("Redis Cache Miss"),
            Err(e) => error!("Redis Get ERROR: {}", e),
        }
    }

    match user_repository.find_by_id(&id).await {
        Ok(Some(user)) => {
            HttpResponse::Ok().json(from_user_entity_to_user_response(user))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(SampleMessage{ message: format!("User {} not found!", id)})
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/{user_id}")]
async fn update_user(
    user_repository: web::Data<UserRepository>,
    path: web::Path<String>,
    payload: Json<UpdateUserRequest>
) -> impl Responder {
    let id = path.into_inner();
    let body = payload.into_inner();
    
    match user_repository.update_by_id(&id, &body).await {
        Ok(updated) => {
            if updated {
                HttpResponse::Ok().json(UpdateUserResponse {
                    updated_user: UserResponse {
                        username: id,
                        name: body.name,
                        age: body.age
                    }
                })
            } else {
                HttpResponse::NotFound().json(SampleMessage{ message: format!("User {} not found!", id)})
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .service(create_user)
        .service(delete_user)
        .service(get_user)
        .service(update_user)
    );
}