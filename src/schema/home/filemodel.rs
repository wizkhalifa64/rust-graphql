use async_graphql::InputObject;
use async_graphql::Json;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, InputObject, Clone)]
pub struct FileBody {
    pub headcount: usize,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject, Clone)]
pub struct Fileresponse {
    pub head: Json<Value>,
    pub describe: Json<Value>,
}
