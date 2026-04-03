use std::collections::HashMap;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::openapi::{
    Component, Info, OpenApi, Operation, PathItem, SchemaOrRef,
    SchemaOrRef::{Inline, Ref},
    SecurityScheme, Server,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenApiSlice {
    pub openapi: String,
    pub info: Info,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    pub paths: HashMap<String, PathItem>,
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

    // TODO refactor when I'm less retarded about rust.
    let comps: Vec<Option<HashMap<String, SchemaOrRef>>> = vec![
        // get
        find_schemas(
            path.cloned(),
            |path_item| path_item.get,
            spec.components.clone(),
            "default",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.get,
            spec.components.clone(),
            "200",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.get,
            spec.components.clone(),
            "404",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.get,
            spec.components.clone(),
            "400",
            "application/json",
        ),
        // post
        find_schemas(
            path.cloned(),
            |path_item| path_item.post,
            spec.components.clone(),
            "default",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.post,
            spec.components.clone(),
            "200",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.post,
            spec.components.clone(),
            "404",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.post,
            spec.components.clone(),
            "400",
            "application/json",
        ),
        // put
        find_schemas(
            path.cloned(),
            |path_item| path_item.put,
            spec.components.clone(),
            "default",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.put,
            spec.components.clone(),
            "200",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.put,
            spec.components.clone(),
            "404",
            "application/json",
        ),
        find_schemas(
            path.cloned(),
            |path_item| path_item.put,
            spec.components.clone(),
            "400",
            "application/json",
        ),
    ];

    OpenApiSlice {
        openapi: spec.openapi.clone(),
        info: spec.info.clone(),
        servers: spec.servers.clone(),
        paths: path_item_slice,
        components: append_schemas(
            spec.components
                .clone()
                .and_then(|component| component.security_schemes),
            comps,
        ),
    }
}

fn get_request_body_schema(
    path_item: PathItem,
    path_item_accessor: fn(PathItem) -> Option<Operation>,
    media_type: &str,
) -> Option<SchemaOrRef> {
    path_item_accessor(path_item)?
        .request_body?
        .content?
        .get(media_type)?
        .schema
        .clone()
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
        .clone()
        .content?
        .get(media_type)?
        .schema
        .clone()
}

fn get_ref_key(schema_ref: &str) -> &str {
    schema_ref.split('/').last().unwrap_or("").trim()
}

fn append_schemas(
    security_scheme: Option<HashMap<String, SecurityScheme>>,
    schemas: Vec<Option<HashMap<String, SchemaOrRef>>>,
) -> Option<Component> {
    let new_schema = schemas.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, SchemaOrRef>, schema| {
            acc.extend(schema.unwrap_or_default());
            acc
        },
    );

    Some(Component {
        security_schemes: security_scheme,
        schemas: Some(new_schema),
    })
}

fn push_ref_from_schema_or_ref(schema_or_ref: &SchemaOrRef, stack: &mut Vec<String>) {
    match schema_or_ref {
        Ref { r#ref } => {
            stack.push(get_ref_key(r#ref).to_string());
        }
        Inline(inline) => {
            if let Some(item) = &inline.items {
                push_ref_from_schema_or_ref(item, stack);
            }

            for a in &inline.any_of {
                push_ref_from_schema_or_ref(a, stack);
            }

            for a in &inline.all_of {
                push_ref_from_schema_or_ref(a, stack);
            }

            if let Some(props) = &inline.properties {
                for p in props.values() {
                    push_ref_from_schema_or_ref(p, stack);
                }
            }
        }
    }
}

fn iter_schema_append(
    key: &str,
    source_schema: HashMap<String, SchemaOrRef>,
) -> HashMap<String, SchemaOrRef> {
    let mut new_schema: HashMap<String, SchemaOrRef> = HashMap::new();
    let mut stack = vec![key.to_string()];

    while let Some(k) = stack.pop() {
        if new_schema.contains_key(&k) {
            continue;
        }
        if let Some(sc) = source_schema.get(&k) {
            new_schema.insert(k.clone(), sc.clone());

            if let Inline(inline) = sc {
                // items field
                if let Some(i) = &inline.items {
                    push_ref_from_schema_or_ref(i, &mut stack);
                }

                // anyOf field
                for any_of in &inline.any_of {
                    push_ref_from_schema_or_ref(any_of, &mut stack);
                }

                // allOf field
                for all_of in &inline.all_of {
                    push_ref_from_schema_or_ref(all_of, &mut stack);
                }

                // properties field
                if let Some(prop) = &inline.properties {
                    for p in prop.values() {
                        push_ref_from_schema_or_ref(p, &mut stack);
                    }
                }
            } else if let Ref { r#ref } = sc {
                stack.push(get_ref_key(r#ref).to_string());
            }
        }
    }
    new_schema
}

fn find_schemas(
    path_item: Option<PathItem>,
    path_item_accessor: fn(PathItem) -> Option<Operation>,
    source_components: Option<Component>,
    response_key: &str,
    media_type: &str,
) -> Option<HashMap<String, SchemaOrRef>> {
    let item = path_item?;

    let response_schema: Option<SchemaOrRef> =
        get_response_schemas(item.clone(), path_item_accessor, response_key, media_type);

    let request_body_schema: Option<SchemaOrRef> =
        get_request_body_schema(item.clone(), path_item_accessor, media_type);

    let source_schema = source_components.clone()?.schemas?;

    let request: HashMap<String, SchemaOrRef> = if let Some(Ref { r#ref }) = request_body_schema {
        let key = get_ref_key(&r#ref);
        iter_schema_append(key, source_schema.clone())
    } else {
        HashMap::new()
    };

    let mut response: HashMap<String, SchemaOrRef> = if let Some(Ref { r#ref }) = response_schema {
        let key = get_ref_key(&r#ref);
        iter_schema_append(key, source_schema.clone())
    } else {
        HashMap::new()
    };

    response.extend(request);
    Some(response)
}

#[allow(dead_code)]
pub fn write_slice_to_file<'a>(path_item: &'a OpenApiSlice, filename: &str) -> io::Result<()> {
    match serde_yaml::to_string(path_item) {
        Err(_) => panic!("Unable to decode path item"),
        Ok(serialized) => fs::write(filename, serialized),
    }
}
