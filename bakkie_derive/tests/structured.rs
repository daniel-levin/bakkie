use bakkie::Structured;

#[bakkie::structured]
enum B {
    Something { field: String },

    Nothing,
}

#[bakkie::structured]
struct S {
    a: String,
    b: B,
}

#[allow(dead_code)]
fn can_json() {
    let _: S = serde_json::from_str("").unwrap();
}

#[test]
fn can_obtain_json_schema() {
    let schema = S::as_json_schema();

    insta::assert_json_snapshot!(schema);
}
