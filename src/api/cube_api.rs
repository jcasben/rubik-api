use crate::{models::cube_model::Cube, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/add_cube", data = "<new_cube>")]
pub fn insert_cube(
    db: &State<MongoRepo>, new_cube: Json<Cube>
) -> Result<Json<InsertOneResult>, Status> {
    let data = Cube {
        id: None,
        name: new_cube.name.to_owned(),
        type_: new_cube.type_.to_owned(),
        wr: new_cube.wr.clone(),
    };
    let cube_detail = db.insert_cube(data);
    match cube_detail {
        Ok(cube) => Ok(Json(cube)),
        Err(_) => Err(Status::InternalServerError),
    }
}