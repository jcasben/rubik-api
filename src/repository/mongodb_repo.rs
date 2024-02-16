use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use crate::models::cube_model::Cube;

pub struct MongoRepo {
    col: Collection<Cube>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(val) => val,
            Err(_) => format!("MONGOURI not found in .env"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rubikapi");
        let col: Collection<Cube> = db.collection("cubes");
        MongoRepo { col }
    }

    pub fn insert_cube(&self, new_cube: Cube) -> Result<InsertOneResult, Error> {
        let cube = Cube {
            id: None,
            name: new_cube.name,
            type_: new_cube.type_,
            wr: new_cube.wr,
        };

        let cube = self
            .col
            .insert_one(cube, None)
            .ok()
            .expect("Failed to insert document.");

        Ok(cube)
    }
}