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
