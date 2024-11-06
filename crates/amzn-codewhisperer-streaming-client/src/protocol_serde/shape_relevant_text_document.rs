// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_relevant_text_document(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RelevantTextDocument,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("relativeFilePath").string(input.relative_file_path.as_str());
    }
    if let Some(var_1) = &input.programming_language {
        #[allow(unused_mut)]
        let mut object_2 = object.key("programmingLanguage").start_object();
        crate::protocol_serde::shape_programming_language::ser_programming_language(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.text {
        object.key("text").string(var_3.as_str());
    }
    if let Some(var_4) = &input.document_symbols {
        let mut array_5 = object.key("documentSymbols").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_document_symbol::ser_document_symbol(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}