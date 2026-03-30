use std::collections::HashMap;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::openapi::{
    Component, Info, OpenApi, PathItem, SchemaOrRef,
    SchemaOrRef::{Inline, Ref},
    Server,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenApiSlice {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub path: HashMap<String, PathItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Component>,
}

#[allow(dead_code)]
pub fn get_path<'a>(spec: &'a OpenApi, pathname: &str) -> OpenApiSlice {
    let path: Option<&PathItem> = spec.paths.get(pathname);
    let mut path_item_slice: HashMap<String, PathItem> = HashMap::new();

    if let Some(path_item) = path {
        path_item_slice.insert(pathname.to_string(), path_item.clone());
    }

    OpenApiSlice {
        openapi: spec.openapi.clone(),
        info: spec.info.clone(),
        servers: spec.servers.clone(),
        path: path_item_slice,
        // TODO components need to depend on whether there is a reference
        components: find_components(path.cloned(), spec.components.clone()),
    }
}

// find components from ref
fn find_components(
    path_item: Option<PathItem>,
    components: Option<Component>,
) -> Option<Component> {
    let item = path_item?;
    let get = item.get?;
    let get_responses_schemas: SchemaOrRef = get
        .responses?
        .get("200")?
        .to_owned()
        .content?
        .get("application/json")?
        .schema
        .to_owned()?;
    let schema = HashMap::new();

    fn mk_component(
        mut new_schema: HashMap<String, SchemaOrRef>,
        str_ref: &str,
        schema_or_ref: SchemaOrRef,
    ) -> Option<Component> {
        match &schema_or_ref {
            Ref { r#ref } => {
                let next_ref = r#ref.split('/').last()?;
                mk_component(new_schema, next_ref, schema_or_ref.to_owned())
            }
            Inline(inline_schema) => {
                new_schema.insert(str_ref.to_string(), Inline(inline_schema.to_owned()));
                Some(Component {
                    schemas: Some(new_schema.to_owned()),
                })
            }
        }
    }

    if let Ref { r#ref } = &get_responses_schemas {
        let str_ref: &str = r#ref.split('/').last()?;
        let schema_or_ref: SchemaOrRef = components.clone()?.schemas?.get(str_ref)?.to_owned();

        let mut component_slice = mk_component(schema, str_ref, schema_or_ref)?;

        let schema = component_slice.schemas.as_mut()?;

        for (key, val) in schema.clone().iter() {
            match val {
                Inline(inline) => {
                    let items_schema_or_ref: SchemaOrRef = *inline.items.to_owned()?;
                    let source_schemas = components.clone()?.schemas?;

                    if let Ref { r#ref } = items_schema_or_ref {
                        let inner_ref = r#ref.split('/').last()?;
                        let inner_schema = source_schemas.get(inner_ref)?;
                        schema.insert(inner_ref.to_string(), inner_schema.clone());
                    }
                }
                Ref { r#ref } => {
                    let r = r#ref.split('/').last()?;
                    let new_component = mk_component(schema.to_owned(), r, val.to_owned())?;
                    let s = &new_component.schemas?;
                    schema.extend(s.iter().map(|(k, v)| (k.clone(), v.clone())));
                }
            }
        }

        Some(Component {
            schemas: Some(schema.to_owned()),
        })
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn write_slice_to_file<'a>(path_item: &'a OpenApiSlice, filename: &str) -> io::Result<()> {
    match serde_yaml::to_string(path_item) {
        Err(_) => panic!("Unable to decode path item"),
        Ok(serialized) => fs::write(filename, serialized),
    }
}
