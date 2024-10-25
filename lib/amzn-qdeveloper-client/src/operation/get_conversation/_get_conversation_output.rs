// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetConversationOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub conversation_id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub messages: ::std::vec::Vec<crate::types::Message>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetConversationOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn conversation_id(&self) -> &str {
        use std::ops::Deref;
        self.conversation_id.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn messages(&self) -> &[crate::types::Message] {
        use std::ops::Deref;
        self.messages.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetConversationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetConversationOutput {
    /// Creates a new builder-style object to manufacture
    /// [`GetConversationOutput`](crate::operation::get_conversation::GetConversationOutput).
    pub fn builder() -> crate::operation::get_conversation::builders::GetConversationOutputBuilder {
        crate::operation::get_conversation::builders::GetConversationOutputBuilder::default()
    }
}

/// A builder for
/// [`GetConversationOutput`](crate::operation::get_conversation::GetConversationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetConversationOutputBuilder {
    pub(crate) conversation_id: ::std::option::Option<::std::string::String>,
    pub(crate) messages: ::std::option::Option<::std::vec::Vec<crate::types::Message>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetConversationOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn conversation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversation_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_conversation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversation_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_conversation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversation_id
    }

    /// Appends an item to `messages`.
    ///
    /// To override the contents of this collection use [`set_messages`](Self::set_messages).
    pub fn messages(mut self, input: crate::types::Message) -> Self {
        let mut v = self.messages.unwrap_or_default();
        v.push(input);
        self.messages = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_messages(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Message>>) -> Self {
        self.messages = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_messages(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Message>> {
        &self.messages
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`GetConversationOutput`](crate::operation::get_conversation::GetConversationOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](crate::operation::get_conversation::builders::GetConversationOutputBuilder::conversation_id)
    /// - [`messages`](crate::operation::get_conversation::builders::GetConversationOutputBuilder::messages)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_conversation::GetConversationOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_conversation::GetConversationOutput {
            conversation_id: self.conversation_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "conversation_id",
                    "conversation_id was not specified but it is required when building GetConversationOutput",
                )
            })?,
            messages: self.messages.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "messages",
                    "messages was not specified but it is required when building GetConversationOutput",
                )
            })?,
            next_token: self.next_token,
            _request_id: self._request_id,
        })
    }
}