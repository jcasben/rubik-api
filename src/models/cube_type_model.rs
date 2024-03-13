use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CubeType {
    Cubic,
    Cuboid,
    Minx,
    Other
}