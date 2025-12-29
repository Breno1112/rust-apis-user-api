use deadpool_redis::{Pool};
use log::info;
use redis::AsyncCommands;

use crate::domain::{entities::mongodb::user_entity::{UserEntity}, exceptions::db::{DatabaseException, DatabaseExceptionType}};

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool
}

impl UserRepository {

    pub fn new(pool: Pool) -> Self {
        UserRepository { pool: pool}
    }

    pub async fn insert_one(&self, user_entity: &UserEntity) -> Result<bool, DatabaseException> {
        match self.pool.get().await {
            Ok(mut conn) => {
                match serde_json::to_string(user_entity) {
                    Ok(serialized_object) => {
                        match conn.set_ex::<_, _, ()>(&user_entity.username, serialized_object, 60).await {
                            Ok(_) => {
                                Ok(true)
                            }
                            Err(_) => {
                                Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
                            }
                        }
                    }
                    Err(_) => {
                        Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
                    }
                }
            }
            Err(_) => {
                Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
            }
        }
    }

    pub async fn delete_by_id(&self, id: &String) -> Result<bool, DatabaseException> {
        match self.pool.get().await {
            Ok(mut conn) => {
                match conn.del::<_, ()>(&id).await {
                    Ok(_) => {
                        info!("deleted from cache");
                        Ok(true)
                    }
                    Err(_) => {
                        Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
                    }
                }
            }
            Err(_) => {
                Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
            }
        }
    }

    pub async fn find_by_id(&self, id: &String) -> Result<Option<UserEntity>, DatabaseException> {
        match self.pool.get().await {
            Ok(mut conn) => {
                match conn.get::<_, Option<String>>(id).await {
                    Ok(Some(res)) => {
                        match serde_json::from_str::<UserEntity>(&res) {
                            Ok(user_entity) => {
                                Ok(Some(user_entity))
                            }
                            Err(_) => {
                                Err(DatabaseException{ message: "Deserialization error".to_string(), kind: DatabaseExceptionType::UnknownError })
                            }
                        }
                    }
                    Ok(None) => {
                        Ok(None)
                    }
                    Err(_) => {
                        Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
                    }
                }
            }
            Err(_) => {
                Err(DatabaseException{ message: "Connection error".to_string(), kind: DatabaseExceptionType::UnknownError })
            }
        }
    }
}