use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{extjson::de::Error, oid::ObjectId, doc}, 
        results::{ InsertOneResult, UpdateResult, DeleteResult},
        Client, Collection,
    };
    use futures::stream::TryStreamExt; //add this
    use crate::models::user_model::User;

    pub struct MongoRepo {
        col: Collection<User>,
    }

    impl MongoRepo {
        pub async fn init() -> Self {
            //init code goes here
        }

        pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
            //create_user code goes here
        }

        pub async fn get_user(&self, id: &String) -> Result<User, Error> {
            //get_user code goes here
        }

        pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
            //update_user code goes here
        }

        pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
            //delete_user code goes here
        }

        pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }
    }