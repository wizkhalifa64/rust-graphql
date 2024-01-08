use crate::schema::scraper::myscrapermodel::AirportDetails;
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};

use super::dberror::MyError;

#[derive(Debug, Clone)]
pub struct DB {
    pub collection: Collection<Document>,
    pub airport_collection: Collection<AirportDetails>,
}

impl DB {
    pub async fn init() -> std::result::Result<Self, MyError> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let collection_name =
            std::env::var("MONGODB_NOTE_COLLECTION").expect("MONGODB_NOTE_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let airport_collection = database.collection(collection_name.as_str());
        let collection = database.collection::<Document>(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            airport_collection,
            collection,
        })
    }
}
