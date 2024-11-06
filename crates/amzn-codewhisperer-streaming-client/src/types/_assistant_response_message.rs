// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Markdown text message.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AssistantResponseMessage {
    /// Unique identifier for the chat message
    pub message_id: ::std::option::Option<::std::string::String>,
    /// The content of the text message in markdown format.
    pub content: ::std::string::String,
    /// Web References
    pub supplementary_web_links: ::std::option::Option<::std::vec::Vec<crate::types::SupplementaryWebLink>>,
    /// Code References
    pub references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
    /// Followup Prompt
    pub followup_prompt: ::std::option::Option<crate::types::FollowupPrompt>,
}
impl AssistantResponseMessage {
    /// Unique identifier for the chat message
    pub fn message_id(&self) -> ::std::option::Option<&str> {
        self.message_id.as_deref()
    }

    /// The content of the text message in markdown format.
    pub fn content(&self) -> &str {
        use std::ops::Deref;
        self.content.deref()
    }

    /// Web References
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.supplementary_web_links.is_none()`.
    pub fn supplementary_web_links(&self) -> &[crate::types::SupplementaryWebLink] {
        self.supplementary_web_links.as_deref().unwrap_or_default()
    }

    /// Code References
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.references.is_none()`.
    pub fn references(&self) -> &[crate::types::Reference] {
        self.references.as_deref().unwrap_or_default()
    }

    /// Followup Prompt
    pub fn followup_prompt(&self) -> ::std::option::Option<&crate::types::FollowupPrompt> {
        self.followup_prompt.as_ref()
    }
}
impl ::std::fmt::Debug for AssistantResponseMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssistantResponseMessage");
        formatter.field("message_id", &self.message_id);
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("supplementary_web_links", &self.supplementary_web_links);
        formatter.field("references", &self.references);
        formatter.field("followup_prompt", &self.followup_prompt);
        formatter.finish()
    }
}
impl AssistantResponseMessage {
    /// Creates a new builder-style object to manufacture
    /// [`AssistantResponseMessage`](crate::types::AssistantResponseMessage).
    pub fn builder() -> crate::types::builders::AssistantResponseMessageBuilder {
        crate::types::builders::AssistantResponseMessageBuilder::default()
    }
}

/// A builder for [`AssistantResponseMessage`](crate::types::AssistantResponseMessage).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct AssistantResponseMessageBuilder {
    pub(crate) message_id: ::std::option::Option<::std::string::String>,
    pub(crate) content: ::std::option::Option<::std::string::String>,
    pub(crate) supplementary_web_links: ::std::option::Option<::std::vec::Vec<crate::types::SupplementaryWebLink>>,
    pub(crate) references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
    pub(crate) followup_prompt: ::std::option::Option<crate::types::FollowupPrompt>,
}
impl AssistantResponseMessageBuilder {
    /// Unique identifier for the chat message
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

    /// The content of the text message in markdown format.
    /// This field is required.
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }

    /// The content of the text message in markdown format.
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }

    /// The content of the text message in markdown format.
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.content
    }

    /// Appends an item to `supplementary_web_links`.
    ///
    /// To override the contents of this collection use
    /// [`set_supplementary_web_links`](Self::set_supplementary_web_links).
    ///
    /// Web References
    pub fn supplementary_web_links(mut self, input: crate::types::SupplementaryWebLink) -> Self {
        let mut v = self.supplementary_web_links.unwrap_or_default();
        v.push(input);
        self.supplementary_web_links = ::std::option::Option::Some(v);
        self
    }

    /// Web References
    pub fn set_supplementary_web_links(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SupplementaryWebLink>>,
    ) -> Self {
        self.supplementary_web_links = input;
        self
    }

    /// Web References
    pub fn get_supplementary_web_links(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::SupplementaryWebLink>> {
        &self.supplementary_web_links
    }

    /// Appends an item to `references`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    ///
    /// Code References
    pub fn references(mut self, input: crate::types::Reference) -> Self {
        let mut v = self.references.unwrap_or_default();
        v.push(input);
        self.references = ::std::option::Option::Some(v);
        self
    }

    /// Code References
    pub fn set_references(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>) -> Self {
        self.references = input;
        self
    }

    /// Code References
    pub fn get_references(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Reference>> {
        &self.references
    }

    /// Followup Prompt
    pub fn followup_prompt(mut self, input: crate::types::FollowupPrompt) -> Self {
        self.followup_prompt = ::std::option::Option::Some(input);
        self
    }

    /// Followup Prompt
    pub fn set_followup_prompt(mut self, input: ::std::option::Option<crate::types::FollowupPrompt>) -> Self {
        self.followup_prompt = input;
        self
    }

    /// Followup Prompt
    pub fn get_followup_prompt(&self) -> &::std::option::Option<crate::types::FollowupPrompt> {
        &self.followup_prompt
    }

    /// Consumes the builder and constructs a
    /// [`AssistantResponseMessage`](crate::types::AssistantResponseMessage). This method will
    /// fail if any of the following fields are not set:
    /// - [`content`](crate::types::builders::AssistantResponseMessageBuilder::content)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::AssistantResponseMessage, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::AssistantResponseMessage {
            message_id: self.message_id,
            content: self.content.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "content",
                    "content was not specified but it is required when building AssistantResponseMessage",
                )
            })?,
            supplementary_web_links: self.supplementary_web_links,
            references: self.references,
            followup_prompt: self.followup_prompt,
        })
    }
}
impl ::std::fmt::Debug for AssistantResponseMessageBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssistantResponseMessageBuilder");
        formatter.field("message_id", &self.message_id);
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("supplementary_web_links", &self.supplementary_web_links);
        formatter.field("references", &self.references);
        formatter.field("followup_prompt", &self.followup_prompt);
        formatter.finish()
    }
}