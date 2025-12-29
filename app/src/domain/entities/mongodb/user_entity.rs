use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")] // MongoDB uses _id for the primary key
    pub username: String,
    pub name: String,
    pub age: u8,
}