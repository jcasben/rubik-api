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
    /// Inits a mongoDB instance with all the info of the database
    /// 
    /// ## Returns
    /// * An instance of a mongoDB repository.
    pub fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI")
            .unwrap_or_else(|_| "MONGOURI not found in .env".to_string());
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rubikapi");
        let col: Collection<Cube> = db.collection("cubes");
        MongoRepo { col }
    }

    /// Inserts a cube into the database.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repo.
    /// * `new_cube` - cube object to be inserted.
    /// 
    /// ## Returns
    /// * The result of the operation.
    pub fn insert_cube(&self, new_cube: Cube) -> Result<InsertOneResult, Error> {
        let cube = Cube {
            id: None,
            name: new_cube.name,
            type_: new_cube.type_,
            pieces: new_cube.pieces,
            faces: new_cube.faces,
            stickers: new_cube.stickers,
            year_created: new_cube.year_created,
            wr: new_cube.wr,
        };

        let cube = self
            .col
            .insert_one(cube, None)
            .expect("Failed to insert document.");

        Ok(cube)
    }

    /// Gets a cube from the database by its ID.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `id` - ID of the cube.
    /// 
    /// ## Returns
    /// * The cube object that was retrieved from the database.
    pub fn get_cube(&self, id: &String) -> Result<Cube, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let cube_detail = self
            .col
            .find_one(filter, None)
            .expect("Error getting cube's detail");
        
        Ok(cube_detail.unwrap())
    }
    
    /// Edits a cube from the database given its ID.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `new_cube` - new cube object definition.
    /// 
    /// ## Returns
    /// * The result of the operation.
    pub fn edit_cube(&self, id: &String, new_cube: Cube) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let bson_type = bson::to_bson(&new_cube.type_).unwrap();
        let bson_wr = bson::to_bson(&new_cube.wr).unwrap();
        let bson_pieces = bson::to_bson(&new_cube.pieces).unwrap();
        let bson_faces  = bson::to_bson(&new_cube.faces).unwrap();
        let bson_stickers  = bson::to_bson(&new_cube.stickers).unwrap();
        let bson_year  = bson::to_bson(&new_cube.year_created).unwrap();
        let new_doc = doc! {
            "$set":
                {
                    "id": new_cube.id,
                    "name": new_cube.name,
                    "type_": bson_type,
                    "pieces": bson_pieces,
                    "faces": bson_faces,
                    "stickers": bson_stickers,
                    "year_created": bson_year,
                    "wr": bson_wr,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .expect("Error updating the cube");

        Ok(updated_doc)
    }
    
    /// Edits a cube from the database given its name.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `new_cube` - new cube object definition.
    /// 
    /// ## Returns
    /// * The result of the operation.
    pub fn edit_cube_by_name(
        &self, 
        name: &String, 
        new_cube: Cube
    ) -> Result<UpdateResult, Error> {
        let filter = doc!{ "name": name };
        let bson_type = bson::to_bson(&new_cube.type_).unwrap();
        let bson_wr = bson::to_bson(&new_cube.wr).unwrap();
        let bson_pieces = bson::to_bson(&new_cube.pieces).unwrap();
        let bson_faces  = bson::to_bson(&new_cube.faces).unwrap();
        let bson_stickers  = bson::to_bson(&new_cube.stickers).unwrap();
        let bson_year  = bson::to_bson(&new_cube.year_created).unwrap();
        let new_doc = doc! {
            "$set":
                {
                    "id": new_cube.id,
                    "name": new_cube.name,
                    "type_": bson_type,
                    "pieces": bson_pieces,
                    "faces": bson_faces,
                    "stickers": bson_stickers,
                    "year_created": bson_year,
                    "wr": bson_wr,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .expect("Error updating the cube");

        Ok(updated_doc)
    }
    
    /// Deletes a cube from the database given its ID.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `id` - ID of the cube to be deleted.
    /// 
    /// ## Returns
    /// * The result of the operation.
    pub fn delete_cube(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let cube_detail = self
            .col
            .delete_one(filter, None)
            .expect("Error deleting the cube");
        
        Ok(cube_detail)
    }
    
    /// Gets all the cubes available from the database.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// 
    /// ## Returns
    /// * A vector that contains all the cubes.
    pub fn get_all_cubes(&self) -> Result<Vec<Cube>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .expect("Error getting list of cubes!");
        let cubes = cursors.map(|doc| doc.unwrap()).collect();

        Ok(cubes)
    }

    /// Gets a cube from the database by its name
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `name` - name of the cube to be retrieved from the database.
    /// 
    /// ## Returns
    /// * The cube object.
    pub fn get_cube_by_name(&self, name: &String) -> Result<Cube, Error> {
        let filter = doc! {"name": name};
        let cube_detail = self
            .col
            .find_one(filter, None)
            .expect("Error getting cube by name!");

        Ok(cube_detail.unwrap())
    }

    /// Gets all the cubes that match the specified type.
    /// 
    /// ## Arguments
    /// * `self` - instance of the mongoDB repository.
    /// * `type_` - type that the cubes must match.
    /// 
    /// ## Returns
    /// A vector with all the cubes that matched the type.
    pub fn get_cube_by_type(&self, type_: &String) -> Result<Vec<Cube>, Error> {
        let filter = doc! {"type_": type_};
        let cursors = self
            .col
            .find(filter, None)
            .expect("Error getting cube by type!");
        let cubes = cursors.map(|doc| doc.unwrap()).collect();

        Ok(cubes)
    }
}