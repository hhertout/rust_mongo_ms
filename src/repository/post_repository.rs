use std::sync::Arc;
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::Bson;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use serde::{Deserialize, Serialize};
use crate::repository::Repository;

pub struct PostRepository {
    client: Client,
    database: String,
    collection: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename="_id", serialize_with = "mongodb::bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub(crate) id: ObjectId,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) content: String,
}

impl PostRepository {
    pub fn new(repository: &Arc<Repository>) -> PostRepository {
        PostRepository {
            client: repository.pool.clone(),
            database: repository.database.clone(),
            collection: String::from("post"),
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Post>, Error> {
        let client = &self.client;
        let collection: Collection<Post> = client.database(&self.database).collection(&self.collection);

        let cursor = collection
            .find(None, None)
            .await?;
        let posts: Vec<Post> = cursor.try_collect()
            .await?;
        Ok(posts)
    }

    pub async fn insert_one(&self, post: NewPost) -> Result<String, Error> {
        let client = &self.client;
        let collection: Collection<NewPost> = client.database(&self.database).collection(&self.collection);

        let res = collection.insert_one(post, None).await.map_err(|err| err)?;
        match res.inserted_id {
            Bson::String(id) => Ok(id),
            Bson::ObjectId(id) => Ok(id.to_hex()),
            _ => Err(Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Internal error"))),
        }
    }
}