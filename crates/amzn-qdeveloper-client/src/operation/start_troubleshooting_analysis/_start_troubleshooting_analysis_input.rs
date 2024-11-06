// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent start troubleshooting analysis request.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartTroubleshootingAnalysisInput {
    #[allow(missing_docs)] // documentation missing in model
    pub error_detail: ::std::option::Option<crate::types::ErrorDetail>,
}
impl StartTroubleshootingAnalysisInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn error_detail(&self) -> ::std::option::Option<&crate::types::ErrorDetail> {
        self.error_detail.as_ref()
    }
}
impl StartTroubleshootingAnalysisInput {
    /// Creates a new builder-style object to manufacture
    /// [`StartTroubleshootingAnalysisInput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput).
    pub fn builder()
    -> crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisInputBuilder {
        crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisInputBuilder::default()
    }
}

/// A builder for
/// [`StartTroubleshootingAnalysisInput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartTroubleshootingAnalysisInputBuilder {
    pub(crate) error_detail: ::std::option::Option<crate::types::ErrorDetail>,
}
impl StartTroubleshootingAnalysisInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn error_detail(mut self, input: crate::types::ErrorDetail) -> Self {
        self.error_detail = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_error_detail(mut self, input: ::std::option::Option<crate::types::ErrorDetail>) -> Self {
        self.error_detail = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_error_detail(&self) -> &::std::option::Option<crate::types::ErrorDetail> {
        &self.error_detail
    }

    /// Consumes the builder and constructs a
    /// [`StartTroubleshootingAnalysisInput`](crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput {
                error_detail: self.error_detail,
            },
        )
    }
}