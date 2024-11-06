// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_message_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::send_message::SendMessageInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.conversation_state {
        #[allow(unused_mut)]
        let mut object_2 = object.key("conversationState").start_object();
        crate::protocol_serde::shape_conversation_state::ser_conversation_state(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.profile_arn {
        object.key("profileArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.source {
        object.key("source").string(var_4.as_str());
    }
    if let Some(var_5) = &input.dry_run {
        object.key("dryRun").boolean(*var_5);
    }
    Ok(())
}