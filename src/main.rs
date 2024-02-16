mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use api::cube_api::insert_cube;
use repository::mongodb_repo::MongoRepo;


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount("/", routes![insert_cube])
}

/*#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world!")))
}*/

/*#[get("/3x3")]
fn get_3x3() -> Json<Cube> {
    Json(Cube {
        name: String::from("3x3"),
        wr: 3.134,
    })
}*/