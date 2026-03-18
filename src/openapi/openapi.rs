use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, PathItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Component>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathItem {
    pub get: Option<Operation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub responses: Option<HashMap<String, Response>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub description: String,
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaType {
    pub schema: Option<SchemaOrRef>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Inline(Schema),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    #[serde(default)]
    pub schemas: Option<HashMap<String, SchemaOrRef>>,
}
