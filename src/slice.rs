use std::collections::HashMap;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::openapi::{
    Component, Info, OpenApi, Operation, PathItem, SchemaOrRef,
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

    let comps = vec![
        find_components(
            path.cloned(),
            |path_item| path_item.get,
            spec.components.clone(),
            "200",
            "application/json",
        ),
        find_components(
            path.clone().cloned(),
            |path_item| path_item.post,
            spec.components.clone(),
            "200",
            "application/json",
        ),
    ];

    OpenApiSlice {
        openapi: spec.openapi.clone(),
        info: spec.info.clone(),
        servers: spec.servers.clone(),
        path: path_item_slice,
        components: append_components(comps),
    }
}

fn get_response_schemas<PathItemAccessor>(
    path_item: PathItem,
    accessor: PathItemAccessor,
    key: &str,
    media_type: &str,
) -> Option<SchemaOrRef>
where
    PathItemAccessor: Fn(PathItem) -> Option<Operation>,
{
    accessor(path_item)?
        .responses?
        .get(key)?
        .to_owned()
        .content?
        .get(media_type)?
        .schema
        .to_owned()
}

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

fn append_components(components: Vec<Option<Component>>) -> Option<Component> {
    let mut root_schema = HashMap::new();
    for component in components {
        if let Some(c) = component {
            if let Some(s) = c.schemas {
                root_schema.extend(s)
            }
        }
    }

    Some(Component {
        schemas: Some(root_schema),
    })
}

// find components from ref
fn find_components<PathItemAccessor>(
    path_item: Option<PathItem>,
    path_item_accessor: PathItemAccessor,
    components: Option<Component>,
    response_key: &str,
    media_type: &str,
) -> Option<Component>
where
    PathItemAccessor: Fn(PathItem) -> Option<Operation>,
{
    let item = path_item?;
    let response_schema = get_response_schemas(item, path_item_accessor, response_key, media_type)?;
    let schema = HashMap::new();

    if let Ref { r#ref } = &response_schema {
        let str_ref: &str = r#ref.split('/').last()?;
        let schema_or_ref: SchemaOrRef = components.clone()?.schemas?.get(str_ref)?.to_owned();

        let mut component_slice = mk_component(schema, str_ref, schema_or_ref)?;

        let schema = component_slice.schemas.as_mut()?;

        for (_key, val) in schema.clone().iter() {
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
