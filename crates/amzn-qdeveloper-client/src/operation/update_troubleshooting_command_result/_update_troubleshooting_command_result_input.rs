// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent update troubleshooting command result request.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct UpdateTroubleshootingCommandResultInput {
    #[allow(missing_docs)] // documentation missing in model
    pub session_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub command_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<crate::types::CommandExecutionStatus>,
    #[allow(missing_docs)] // documentation missing in model
    pub result: ::std::option::Option<::std::string::String>,
}
impl UpdateTroubleshootingCommandResultInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn session_id(&self) -> ::std::option::Option<&str> {
        self.session_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn command_id(&self) -> ::std::option::Option<&str> {
        self.command_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<&crate::types::CommandExecutionStatus> {
        self.status.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn result(&self) -> ::std::option::Option<&str> {
        self.result.as_deref()
    }
}
impl ::std::fmt::Debug for UpdateTroubleshootingCommandResultInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateTroubleshootingCommandResultInput");
        formatter.field("session_id", &self.session_id);
        formatter.field("command_id", &self.command_id);
        formatter.field("status", &self.status);
        formatter.field("result", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl UpdateTroubleshootingCommandResultInput {
    /// Creates a new builder-style object to manufacture
    /// [`UpdateTroubleshootingCommandResultInput`](crate::operation::update_troubleshooting_command_result::UpdateTroubleshootingCommandResultInput).
    pub fn builder()
    -> crate::operation::update_troubleshooting_command_result::builders::UpdateTroubleshootingCommandResultInputBuilder
    {
        crate::operation::update_troubleshooting_command_result::builders::UpdateTroubleshootingCommandResultInputBuilder::default()
    }
}

/// A builder for
/// [`UpdateTroubleshootingCommandResultInput`](crate::operation::update_troubleshooting_command_result::UpdateTroubleshootingCommandResultInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct UpdateTroubleshootingCommandResultInputBuilder {
    pub(crate) session_id: ::std::option::Option<::std::string::String>,
    pub(crate) command_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CommandExecutionStatus>,
    pub(crate) result: ::std::option::Option<::std::string::String>,
}
impl UpdateTroubleshootingCommandResultInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.session_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.session_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.session_id
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn command_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.command_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_command_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.command_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_command_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.command_id
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn status(mut self, input: crate::types::CommandExecutionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CommandExecutionStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CommandExecutionStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn result(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.result = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_result(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.result = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_result(&self) -> &::std::option::Option<::std::string::String> {
        &self.result
    }

    /// Consumes the builder and constructs a
    /// [`UpdateTroubleshootingCommandResultInput`](crate::operation::update_troubleshooting_command_result::UpdateTroubleshootingCommandResultInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_troubleshooting_command_result::UpdateTroubleshootingCommandResultInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_troubleshooting_command_result::UpdateTroubleshootingCommandResultInput {
                session_id: self.session_id,
                command_id: self.command_id,
                status: self.status,
                result: self.result,
            },
        )
    }
}
impl ::std::fmt::Debug for UpdateTroubleshootingCommandResultInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateTroubleshootingCommandResultInputBuilder");
        formatter.field("session_id", &self.session_id);
        formatter.field("command_id", &self.command_id);
        formatter.field("status", &self.status);
        formatter.field("result", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}