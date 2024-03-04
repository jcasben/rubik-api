mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use api::cube_api::{insert_cube, get_cube, delete_cube, update_cube, get_all_cubes};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 10000)))
        .manage(db)
        .mount("/", routes![insert_cube])
        .mount("/", routes![get_cube])
        .mount("/", routes![update_cube])
        .mount("/", routes![delete_cube])
        .mount("/", routes![get_all_cubes])
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