use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, PathItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Component>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Info {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Server {
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct PathItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub responses: Option<HashMap<String, Response>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Response {
    pub description: String,
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct MediaType {
    pub schema: Option<SchemaOrRef>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Inline(Schema),
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Schema {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SchemaOrRef>>,

    #[serde(default, rename = "allOf", skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<SchemaOrRef>,

    #[serde(default, rename = "anyOf", skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<SchemaOrRef>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Component {
    #[serde(default)]
    pub schemas: Option<HashMap<String, SchemaOrRef>>,
}

#[allow(dead_code)]
pub fn decode_spec<'a>(raw_spec: &'a String) -> OpenApi {
    match serde_yaml::from_str(raw_spec) {
        Err(err) => panic!("Unable to decode spec. Error {:?}", err),
        Ok(result) => result,
    }
}
