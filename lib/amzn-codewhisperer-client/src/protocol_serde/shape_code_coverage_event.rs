// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_code_coverage_event(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CodeCoverageEvent,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.customization_arn {
        object.key("customizationArn").string(var_1.as_str());
    }
    {
        #[allow(unused_mut)]
        let mut object_2 = object.key("programmingLanguage").start_object();
        crate::protocol_serde::shape_programming_language::ser_programming_language(
            &mut object_2,
            &input.programming_language,
        )?;
        object_2.finish();
    }
    {
        object.key("acceptedCharacterCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.accepted_character_count).into()),
        );
    }
    {
        object.key("totalCharacterCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.total_character_count).into()),
        );
    }
    {
        object
            .key("timestamp")
            .date_time(&input.timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if input.unmodified_accepted_character_count != 0 {
        object.key("unmodifiedAcceptedCharacterCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.unmodified_accepted_character_count).into()),
        );
    }
    Ok(())
}