use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::oid::ObjectId;
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
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAirportDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
