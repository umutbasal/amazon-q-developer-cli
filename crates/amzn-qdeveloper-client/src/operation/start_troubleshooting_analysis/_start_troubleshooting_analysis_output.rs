// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent start troubleshooting analysis response.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartTroubleshootingAnalysisOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub session_id: ::std::string::String,
    _request_id: Option<String>,
}
impl StartTroubleshootingAnalysisOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn session_id(&self) -> &str {
        use std::ops::Deref;
        self.session_id.deref()
    }
}
impl ::aws_types::request_id::RequestId for StartTroubleshootingAnalysisOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartTroubleshootingAnalysisOutput {
    /// Creates a new builder-style object to manufacture
    /// [`StartTroubleshootingAnalysisOutput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisOutput).
    pub fn builder()
    -> crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisOutputBuilder {
        crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisOutputBuilder::default()
    }
}

/// A builder for
/// [`StartTroubleshootingAnalysisOutput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartTroubleshootingAnalysisOutputBuilder {
    pub(crate) session_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl StartTroubleshootingAnalysisOutputBuilder {
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

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`StartTroubleshootingAnalysisOutput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisOutputBuilder::session_id)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisOutput {
            session_id: self.session_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "session_id",
                    "session_id was not specified but it is required when building StartTroubleshootingAnalysisOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}