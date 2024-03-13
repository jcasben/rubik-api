use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CubeType {
    Cubic,
    Cuboid,
    Minx,
    Other
}