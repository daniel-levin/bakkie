use std::fs;

use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> anyhow::Result<()> {
    let content = include_str!("../../schema/2025-03-26.json");
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(content)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let contents = type_space.to_stream().to_string();

    fs::write("candidate.rs", contents)?;

    Ok(())
}
