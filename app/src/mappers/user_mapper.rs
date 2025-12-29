use crate::domain::{entities::mongodb::user_entity::UserEntity, objects::{request::user::CreateUserRequest, response::user::CreateUserResponse}};


pub fn from_create_user_request_to_user_entity(o1: CreateUserRequest) -> UserEntity {
    UserEntity { username: o1.username, name: o1.name, age: o1.age }
}

pub fn from_user_entity_to_create_user_response(o1: UserEntity) -> CreateUserResponse {
    CreateUserResponse { username: o1.username, name: o1.name, age: o1.age }
}