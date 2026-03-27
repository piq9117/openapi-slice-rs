use glob::glob;
use openapi_rs::openapi::OpenApi;

#[test]
fn openapi_roundrip() {
    for entry in glob("./samples/*.yaml").unwrap().filter_map(Result::ok) {
        let content = std::fs::read_to_string(&entry).unwrap();
        let spec: OpenApi = serde_yaml::from_str(&content).unwrap();
        let re_encoded = serde_yaml::to_string(&spec).unwrap();
        let re_decoded = serde_yaml::from_str(&re_encoded).unwrap();
        assert_eq!(spec, re_decoded);
    }
    // TODO
    // re-evaluate golden test.
    // first attempt with golden test using insta failed.
    // order is not preserved so the test never passes.
}
