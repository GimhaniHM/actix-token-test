use std::env;
extern crate dotenv;
use dotenv::dotenv;

// use futures::TryStreamExt;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult, //, UpdateResult, DeleteResult //modify here
    Client, Collection,
    bson::doc,
    bson,
};

use crate::models::user_model::User;

#[derive(Clone)]
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {

    pub async fn init() -> Self {

        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("test-db");
        let col: Collection<User> = db.collection("User");

        println!("✅ Database connected successfully");

        MongoRepo { 
            col
        }
        
    }

    pub async fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let filter = doc! {"email": email}; // Define the filter for the email field
        let exit_user = self.col.find_one(doc! {"email": email}, None).await.unwrap();
        match exit_user{
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(Some(model)),
                Err(e) => Err(Error::from(e)),
            },
            Ok(None) => Ok(None),
        }
    

        /*
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());
        let cursor = db
            .collection(collection_name.as_str())
            .find_one(doc! {"email": email}, None)
            .unwrap();
        match cursor {
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(model),
                Err(e) => Err(Error::from(e)),
            },
            None => Ok(None),
        }  */
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {

        let doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            pwd: new_user.pwd,
            location: new_user.location
        };

        let user = self
            .col
            .insert_one(doc, None)
            .await.ok()
            .expect("Error creating user");

        Ok(user)
    }

    // pub async fn get_user(&self, id: &String) -> Result<User, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let user_detail = self
    //         .col
    //         .find_one(filter, None)
    //         .await
    //         .ok()
    //         .expect("Error getting user's detail");
    //     Ok(user_detail.unwrap())
    // }

}