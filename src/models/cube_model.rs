use mongodb::bson::oid::ObjectId;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Cube {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>,
    pub name: String,
    pub type_: String,
    pub wr: WorldRecord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldRecord {
    pub author: String,
    pub time: f32,
    pub date: String,
}