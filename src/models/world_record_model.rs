use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldRecord {
    pub author: String,
    pub time: f32,
    pub date: String,
}