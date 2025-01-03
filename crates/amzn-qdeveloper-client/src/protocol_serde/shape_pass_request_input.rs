// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pass_request_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::pass_request::PassRequestInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.extension_fas_policy_path {
        object.key("extensionFasPolicyPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.extension_kms_key_arn {
        object.key("extensionKmsKeyArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tools {
        let mut array_4 = object.key("tools").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tool::ser_tool(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}
