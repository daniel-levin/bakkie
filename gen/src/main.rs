use std::{env, fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> anyhow::Result<()> {
    let content = include_str!("../../schema/2026-06-18.json");
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let contents = type_space.to_stream().to_string();

    fs::write("candidate.rs", contents)?;

    Ok(())
}
