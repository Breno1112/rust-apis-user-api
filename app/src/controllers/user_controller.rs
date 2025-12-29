use actix_web::{HttpResponse, Responder, post, web};
use mongodb::Client;

use crate::domain::entities::mongodb::user_entity::User;

#[post("")]
async fn create_user(
    mongo_client: web::Data<Client>,
    payload: web::Json<User>
) -> impl Responder {
    println!("insert user endpoint called");
    let collection = mongo_client.database("fachinis").collection::<User>("users");

    let user = payload.into_inner();
    match collection.insert_one(&user).await {
        Ok(result) => {
            println!("user inserted! UID is {}", result.inserted_id);
            HttpResponse::Created().json(user)
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
    );
}