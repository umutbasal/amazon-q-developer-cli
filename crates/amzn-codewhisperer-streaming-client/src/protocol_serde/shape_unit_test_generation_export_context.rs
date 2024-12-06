// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_unit_test_generation_export_context(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UnitTestGenerationExportContext,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("testGenerationJobGroupName")
            .string(input.test_generation_job_group_name.as_str());
    }
    if let Some(var_1) = &input.test_generation_job_id {
        object.key("testGenerationJobId").string(var_1.as_str());
    }
    Ok(())
}