// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent code transformation response.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartTestGenerationOutput {
    /// Represents a test generation job
    pub test_generation_job: ::std::option::Option<crate::types::TestGenerationJob>,
    _request_id: Option<String>,
}
impl StartTestGenerationOutput {
    /// Represents a test generation job
    pub fn test_generation_job(&self) -> ::std::option::Option<&crate::types::TestGenerationJob> {
        self.test_generation_job.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for StartTestGenerationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartTestGenerationOutput {
    /// Creates a new builder-style object to manufacture
    /// [`StartTestGenerationOutput`](crate::operation::start_test_generation::StartTestGenerationOutput).
    pub fn builder() -> crate::operation::start_test_generation::builders::StartTestGenerationOutputBuilder {
        crate::operation::start_test_generation::builders::StartTestGenerationOutputBuilder::default()
    }
}

/// A builder for
/// [`StartTestGenerationOutput`](crate::operation::start_test_generation::StartTestGenerationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartTestGenerationOutputBuilder {
    pub(crate) test_generation_job: ::std::option::Option<crate::types::TestGenerationJob>,
    _request_id: Option<String>,
}
impl StartTestGenerationOutputBuilder {
    /// Represents a test generation job
    pub fn test_generation_job(mut self, input: crate::types::TestGenerationJob) -> Self {
        self.test_generation_job = ::std::option::Option::Some(input);
        self
    }

    /// Represents a test generation job
    pub fn set_test_generation_job(mut self, input: ::std::option::Option<crate::types::TestGenerationJob>) -> Self {
        self.test_generation_job = input;
        self
    }

    /// Represents a test generation job
    pub fn get_test_generation_job(&self) -> &::std::option::Option<crate::types::TestGenerationJob> {
        &self.test_generation_job
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
    /// [`StartTestGenerationOutput`](crate::operation::start_test_generation::StartTestGenerationOutput).
    pub fn build(self) -> crate::operation::start_test_generation::StartTestGenerationOutput {
        crate::operation::start_test_generation::StartTestGenerationOutput {
            test_generation_job: self.test_generation_job,
            _request_id: self._request_id,
        }
    }
}