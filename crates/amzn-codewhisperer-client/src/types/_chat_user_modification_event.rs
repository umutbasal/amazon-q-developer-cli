// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ChatUserModificationEvent {
    /// ID which represents a multi-turn conversation
    pub conversation_id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub customization_arn: ::std::option::Option<::std::string::String>,
    /// Unique identifier for the chat message
    pub message_id: ::std::string::String,
    /// Programming Languages supported by CodeWhisperer
    pub programming_language: ::std::option::Option<crate::types::ProgrammingLanguage>,
    #[allow(missing_docs)] // documentation missing in model
    pub modification_percentage: f64,
    #[allow(missing_docs)] // documentation missing in model
    pub has_project_level_context: ::std::option::Option<bool>,
}
impl ChatUserModificationEvent {
    /// ID which represents a multi-turn conversation
    pub fn conversation_id(&self) -> &str {
        use std::ops::Deref;
        self.conversation_id.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn customization_arn(&self) -> ::std::option::Option<&str> {
        self.customization_arn.as_deref()
    }

    /// Unique identifier for the chat message
    pub fn message_id(&self) -> &str {
        use std::ops::Deref;
        self.message_id.deref()
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn programming_language(&self) -> ::std::option::Option<&crate::types::ProgrammingLanguage> {
        self.programming_language.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn modification_percentage(&self) -> f64 {
        self.modification_percentage
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn has_project_level_context(&self) -> ::std::option::Option<bool> {
        self.has_project_level_context
    }
}
impl ChatUserModificationEvent {
    /// Creates a new builder-style object to manufacture
    /// [`ChatUserModificationEvent`](crate::types::ChatUserModificationEvent).
    pub fn builder() -> crate::types::builders::ChatUserModificationEventBuilder {
        crate::types::builders::ChatUserModificationEventBuilder::default()
    }
}

/// A builder for [`ChatUserModificationEvent`](crate::types::ChatUserModificationEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ChatUserModificationEventBuilder {
    pub(crate) conversation_id: ::std::option::Option<::std::string::String>,
    pub(crate) customization_arn: ::std::option::Option<::std::string::String>,
    pub(crate) message_id: ::std::option::Option<::std::string::String>,
    pub(crate) programming_language: ::std::option::Option<crate::types::ProgrammingLanguage>,
    pub(crate) modification_percentage: ::std::option::Option<f64>,
    pub(crate) has_project_level_context: ::std::option::Option<bool>,
}
impl ChatUserModificationEventBuilder {
    /// ID which represents a multi-turn conversation
    /// This field is required.
    pub fn conversation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversation_id = ::std::option::Option::Some(input.into());
        self
    }

    /// ID which represents a multi-turn conversation
    pub fn set_conversation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversation_id = input;
        self
    }

    /// ID which represents a multi-turn conversation
    pub fn get_conversation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversation_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn customization_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customization_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_customization_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customization_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_customization_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.customization_arn
    }

    /// Unique identifier for the chat message
    /// This field is required.
    pub fn message_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Unique identifier for the chat message
    pub fn set_message_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_id = input;
        self
    }

    /// Unique identifier for the chat message
    pub fn get_message_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_id
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn programming_language(mut self, input: crate::types::ProgrammingLanguage) -> Self {
        self.programming_language = ::std::option::Option::Some(input);
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn set_programming_language(mut self, input: ::std::option::Option<crate::types::ProgrammingLanguage>) -> Self {
        self.programming_language = input;
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn get_programming_language(&self) -> &::std::option::Option<crate::types::ProgrammingLanguage> {
        &self.programming_language
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn modification_percentage(mut self, input: f64) -> Self {
        self.modification_percentage = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_modification_percentage(mut self, input: ::std::option::Option<f64>) -> Self {
        self.modification_percentage = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_modification_percentage(&self) -> &::std::option::Option<f64> {
        &self.modification_percentage
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn has_project_level_context(mut self, input: bool) -> Self {
        self.has_project_level_context = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_has_project_level_context(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_project_level_context = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_has_project_level_context(&self) -> &::std::option::Option<bool> {
        &self.has_project_level_context
    }

    /// Consumes the builder and constructs a
    /// [`ChatUserModificationEvent`](crate::types::ChatUserModificationEvent). This method will
    /// fail if any of the following fields are not set:
    /// - [`conversation_id`](crate::types::builders::ChatUserModificationEventBuilder::conversation_id)
    /// - [`message_id`](crate::types::builders::ChatUserModificationEventBuilder::message_id)
    /// - [`modification_percentage`](crate::types::builders::ChatUserModificationEventBuilder::modification_percentage)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::ChatUserModificationEvent, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::ChatUserModificationEvent {
            conversation_id: self.conversation_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "conversation_id",
                    "conversation_id was not specified but it is required when building ChatUserModificationEvent",
                )
            })?,
            customization_arn: self.customization_arn,
            message_id: self.message_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "message_id",
                    "message_id was not specified but it is required when building ChatUserModificationEvent",
                )
            })?,
            programming_language: self.programming_language,
            modification_percentage: self.modification_percentage.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "modification_percentage",
                    "modification_percentage was not specified but it is required when building ChatUserModificationEvent",
                )
            })?,
            has_project_level_context: self.has_project_level_context,
        })
    }
}