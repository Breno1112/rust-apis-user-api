use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub age: u8,
}