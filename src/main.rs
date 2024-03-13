mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use std::env;
use dotenv::dotenv;
use api::cube_api::*;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT not found in .env");
    let address = env::var("ADDRESS").unwrap_or_else(|_| "ADDRESS not found in .env".to_string());
    let db = MongoRepo::init();
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)).merge(("address", address)))
        .manage(db)
        .mount("/", routes![insert_cube])
        .mount("/", routes![get_cube])
        .mount("/", routes![update_cube])
        .mount("/", routes![delete_cube])
        .mount("/", routes![get_all_cubes])
        .mount("/", routes![get_cube_by_name])
        .mount("/", routes![get_cube_by_type])
}