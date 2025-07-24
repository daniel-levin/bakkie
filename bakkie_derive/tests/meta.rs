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
}

/// I want her to know it was me.
#[bakkie::tool]
async fn name_with_annotations(first_name: String, last_name: String) -> Result<(), ToolError> {
    let _ = first_name;
    let _ = last_name;
    Ok(())
}

#[test]
fn test_name_with_annotations() {
    let p = name_with_annotations_particulars();

    assert_eq!(p.description.unwrap(), " I want her to know it was me.");
    assert!(p.title.is_none());
    assert_eq!(p.name, "name_with_annotations");
}

/// The docstring should lose.
#[bakkie::tool(description = "the punctuated list should win")]
async fn docstring_loses(first_name: String, last_name: String) -> Result<(), ToolError> {
    let _ = first_name;
    let _ = last_name;
    Ok(())
}

#[test]
fn test_docstring_loses() {
    let p = docstring_loses_particulars();

    assert_eq!(p.description.unwrap(), "the punctuated list should win");
    assert!(p.title.is_none());
    assert_eq!(p.name, "docstring_loses");
}

/// The docstring should lose.
#[bakkie::tool(title = "foo", description = "the punctuated list should win")]
async fn docstring_loses2(first_name: String, last_name: String) -> Result<(), ToolError> {
    let _ = first_name;
    let _ = last_name;
    Ok(())
}

#[test]
fn test_docstring_loses2() {
    let p = docstring_loses2_particulars();

    assert_eq!(p.description.unwrap(), "the punctuated list should win");
    assert_eq!(p.title.unwrap(), "foo");
    assert_eq!(p.name, "docstring_loses2");
}
