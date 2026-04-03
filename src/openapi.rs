use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct OpenApi {
    pub openapi: String,

    pub info: Info,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

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

    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,

    pub responses: Option<HashMap<String, Response>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct RequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Response {
    pub description: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    pub properties: Option<HashMap<String, SchemaOrRef>>,

    #[serde(default, rename = "maxLength", skip_serializing_if = "Option::is_none")]
    max_length: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Component {
    #[serde(default)]
    pub schemas: Option<HashMap<String, SchemaOrRef>>,

    #[serde(
        default,
        rename = "securitySchemes",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct SecurityScheme {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[allow(dead_code)]
pub fn decode_spec<'a>(raw_spec: &'a String) -> OpenApi {
    match serde_yaml::from_str(raw_spec) {
        Err(err) => panic!("Unable to decode spec. Error {:?}", err),
        Ok(result) => result,
    }
}
