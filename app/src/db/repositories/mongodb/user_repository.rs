use std::fmt::format;

use log::error;
use mongodb::bson::Document;
use mongodb::{Client, Collection};
use mongodb::{bson::doc};

use crate::domain::objects::request::user::UpdateUserRequest;
use crate::domain::{entities::mongodb::user_entity::UserEntity, exceptions::db::{ DatabaseException, DatabaseExceptionType }};

const DATABASE: &str = "fachinis";
const USER_COLLECTION: &str = "users";


#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<UserEntity>
}

impl UserRepository {
    pub fn new(client: &Client) -> Self {
        let collection = client.database(DATABASE).collection::<UserEntity>(USER_COLLECTION);
        UserRepository { collection }
    }

    pub async fn insert_user(&self, user_entity: UserEntity) -> Result<UserEntity, DatabaseException> {
        match self.collection.insert_one(&user_entity).await {
            Ok(_) => {
                Ok(user_entity)
            }
            Err(e) => {
                error!("Error when inserting user {}. The error is {}", e, user_entity.username);
                Err(DatabaseException{message: "Error creating user".to_string(), kind: DatabaseExceptionType::UnknownError})
            }
        }
    }

    pub async fn delete_by_id(&self, id: &String) -> Result<bool, DatabaseException>{
        let filter = doc! {"_id": id};
        match self.collection.delete_one(filter).await {
            Ok(delete_result) => {
                Ok(delete_result.deleted_count > 0)
            }
            Err(e) => {
                error!("Error when deleting user. Error is {}", e.to_string());
                Err(DatabaseException { message: format(format_args!("Unable to delete user {}", id.to_string())), kind: DatabaseExceptionType::UnknownError })
            }
        }
    }

    pub async fn find_by_id(&self, id: &String) -> Result<Option<UserEntity>, DatabaseException> {
        let filter = doc! {"_id": &id};
        match self.collection.find_one(filter).await {
            Ok(Some(user)) => {
                Ok(Some(user))
            }
            Ok(None) => {
                Ok(Option::None)
            }
            Err(e) => {
                error!("Error when finding user. Error is {}", e.to_string());
                Err(DatabaseException{message: format(format_args!("Error when finding user. Error is {}", e.to_string())), kind: DatabaseExceptionType::UnknownError})
            }
        }
    }

    pub async fn update_by_id(&self, id: &String, update_request: &UpdateUserRequest) -> Result<bool, DatabaseException> {
        let filter = doc! {"_id": &id};

        let mut set_doc = Document::new();

        // "name": &body.name,
        //         "age": body.age as i32
        if "" != update_request.name {

            set_doc.insert("name", &update_request.name);
        }
        if 0 != update_request.age {
            set_doc.insert("age", update_request.age as i32);
        }

        if set_doc.is_empty() {
            return Ok(false);
        }

        let update = doc! {
            "$set": set_doc
        };

        match self.collection.update_one(filter, update).await {
            Ok(result) => {
                if result.matched_count == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                Err(DatabaseException{message: format(format_args!("Error when updating user. Error is {}", e.to_string())), kind: DatabaseExceptionType::UnknownError})
            }
        }
    }
}