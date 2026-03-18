use glob::glob;
use openapi_rs::openapi::openapi::OpenApi;

#[test]
fn openapi_roundrip() {
    for entry in glob("./samples/*.yaml").unwrap().filter_map(Result::ok) {
        let content = std::fs::read_to_string(&entry).unwrap();
        let spec: OpenApi = serde_yaml::from_str(&content).unwrap();
        insta::assert_yaml_snapshot!(spec);
    }
}
