use bakkie::provisions::tools::ToolError;

#[bakkie::tool]
async fn only_name(first_name: String, last_name: String) -> Result<(), ToolError> {
    let _ = first_name;
    let _ = last_name;
    Ok(())
}

#[test]
fn test_only_name() {
    let p = only_name_particulars();

    assert!(p.description.is_none());
    assert!(p.title.is_none());
    assert_eq!(p.name, "only_name");
    assert!(p.output_schema.is_none());
}
