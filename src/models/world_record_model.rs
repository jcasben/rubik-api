use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldRecord {
    pub author: String,
    pub time: String,
    pub date: String,
}