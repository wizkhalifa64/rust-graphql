use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct ScraperBody {
    pub airport: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct ScraperRes {
    pub response: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AirportDetails {
    pub title: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    pub city: Option<String>,
}
