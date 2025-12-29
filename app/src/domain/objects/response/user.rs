use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub created_user: UserResponse
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub name: String,
    pub age: u8,
}