use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
}
