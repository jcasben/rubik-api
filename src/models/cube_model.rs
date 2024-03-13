use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::cube_type_model::CubeType;
use crate::models::world_record_model::WorldRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cube {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>,
    pub name: String,
    pub type_: CubeType,
    pub pieces: u32,
    pub faces: u32,
    pub stickers: u32,
    pub year_created: u32,
    pub wr: WorldRecord,
}