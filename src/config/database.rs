use std::env;

pub async fn connect() -> mongodb::Client {
    let uri = env::var("MONGO_URL").unwrap_or_else(|_| panic!("Env var MONGO_URL is not set"));

    let client = match mongodb::Client::with_uri_str(uri).await {
        Ok(client) => client,
        Err(_) => panic!("Failed to connect to database")
    };

    return client;
}