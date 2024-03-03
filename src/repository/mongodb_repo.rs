use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    bson, 
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection}
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
            .expect("Failed to insert document.");

        Ok(cube)
    }

    pub fn get_cube(&self, id: &String) -> Result<Cube, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .expect("Error getting cube's detail");
        
        Ok(user_detail.unwrap())
    }
    
    pub fn edit_cube(&self, id: &String, new_cube: Cube) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filer = doc! {"_id": obj_id};
        let bson_wr = bson::to_bson(&new_cube.wr).unwrap();
        let new_doc = doc! {
            "$set":
                {
                    "id": new_cube.id,
                    "name": new_cube.name,
                    "type_": new_cube.type_,
                    "wr": bson_wr,
                },
        };
        let updated_doc = self
            .col
            .update_one(filer, new_doc, None)
            .expect("Error updating the cube");

        Ok(updated_doc)
    }
    
    pub fn delete_cube(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let cube_detail = self
            .col
            .delete_one(filter, None)
            .expect("Error deleting the cube");
        
        Ok(cube_detail)
    }
    
    pub fn get_all_cubes(&self) -> Result<Vec<Cube>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .expect("Error getting list of cubes!");
        let cubes = cursors.map(|doc| doc.unwrap()).collect();
        
        Ok(cubes)
    }
}