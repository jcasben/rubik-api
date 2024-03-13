mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use api::cube_api::*;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 10000)).merge(("address", "0.0.0.0")))
        .manage(db)
        .mount("/", routes![insert_cube])
        .mount("/", routes![get_cube])
        .mount("/", routes![update_cube])
        .mount("/", routes![delete_cube])
        .mount("/", routes![get_all_cubes])
        .mount("/", routes![get_cube_by_name])
        .mount("/", routes![get_cube_by_type])
}