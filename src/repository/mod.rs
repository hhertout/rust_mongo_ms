use std::env;
use mongodb::Client;
use crate::config::database;

pub mod post_repository;

pub struct Repository {
    pool: Client,
    database: String,
}

impl Repository {
    pub async fn new() -> Repository {
        let database = env::var("MONGO_DB").unwrap_or_else(|_| panic!("MONGO_DB env var is not set"));
        Repository {
            pool: database::connect().await,
            database,
        }
    }
}