use bakkie::tool::Tool;
use schemars::{JsonSchema, schema_for};
use serde::Serialize;

#[test]
fn serialize() {
    #[derive(Serialize, JsonSchema)]
    struct City {
        location: String,
    }

    let s = schema_for!(City);

    let t = Tool {
        name: "get_weather".into(),
        title: "Weather Information Provider".into(),
        description: "Get current weather information".into(),
        input_schema: s,
        construct_fn: |x| Box::pin(async { unimplemented!() }),
    };

    insta::assert_json_snapshot!(t);
}
