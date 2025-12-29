use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub username: String,
    pub name: String,
    pub age: u8,
}