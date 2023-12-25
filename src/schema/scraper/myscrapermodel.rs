use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct ScraperBody {
    pub airport: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct ScraperRes {
    pub response: Value,
}
