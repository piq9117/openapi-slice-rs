use serde_yaml;
use std::fs::read_to_string;

mod openapi;
use openapi::openapi::OpenApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openapi_spec = read_to_string("./samples/all-of.yaml")?;
    let decoded_spec: OpenApi = serde_yaml::from_str(&openapi_spec)?;
    println!("{:#?}", decoded_spec);
    Ok(())
}
