// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_document_symbol(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DocumentSymbol,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    if let Some(var_1) = &input.source {
        object.key("source").string(var_1.as_str());
    }
    Ok(())
}