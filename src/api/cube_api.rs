use crate::{models::cube_model::Cube, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
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

#[get("/cube?<id>")]
pub fn get_cube(db: &State<MongoRepo>, id: String) -> Result<Json<Cube>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let cube_detail = db.get_cube(&id);
    match cube_detail {
        Ok(cube) => Ok(Json(cube)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/cube?<id>", data = "<new_cube>")]
pub fn update_cube(
    db: &State<MongoRepo>, id: String, new_cube: Json<Cube>, 
) -> Result<Json<Cube>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = Cube {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_cube.name.to_owned(),
        type_: new_cube.type_.to_owned(),
        wr: new_cube.wr.clone(),
    };
    
    let update_result = db.edit_cube(&id, data);
    match update_result { 
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_cube_info = db.get_cube(&id);
                match updated_cube_info {
                    Ok(cube) => Ok(Json(cube)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/delete_cube?<id>")]
pub fn delete_cube(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_cube(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                Ok(Json("Cube successfully deleted!"))
            } else {
                Err(Status::InternalServerError)
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/cubes")]
pub fn get_all_cubes(db: &State<MongoRepo>) -> Result<Json<Vec<Cube>>, Status> {
    let cubes = db.get_all_cubes();
    match cubes {
        Ok(cubes) => Ok(Json(cubes)),
        Err(_) => Err(Status::InternalServerError),
    }
}