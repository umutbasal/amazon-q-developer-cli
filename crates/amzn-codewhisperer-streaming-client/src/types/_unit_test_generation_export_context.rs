// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Unit test generation export context
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UnitTestGenerationExportContext {
    /// Test generation job group name
    pub test_generation_job_group_name: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub test_generation_job_id: ::std::option::Option<::std::string::String>,
}
impl UnitTestGenerationExportContext {
    /// Test generation job group name
    pub fn test_generation_job_group_name(&self) -> &str {
        use std::ops::Deref;
        self.test_generation_job_group_name.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn test_generation_job_id(&self) -> ::std::option::Option<&str> {
        self.test_generation_job_id.as_deref()
    }
}
impl UnitTestGenerationExportContext {
    /// Creates a new builder-style object to manufacture
    /// [`UnitTestGenerationExportContext`](crate::types::UnitTestGenerationExportContext).
    pub fn builder() -> crate::types::builders::UnitTestGenerationExportContextBuilder {
        crate::types::builders::UnitTestGenerationExportContextBuilder::default()
    }
}

/// A builder for
/// [`UnitTestGenerationExportContext`](crate::types::UnitTestGenerationExportContext).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UnitTestGenerationExportContextBuilder {
    pub(crate) test_generation_job_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) test_generation_job_id: ::std::option::Option<::std::string::String>,
}
impl UnitTestGenerationExportContextBuilder {
    /// Test generation job group name
    /// This field is required.
    pub fn test_generation_job_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_generation_job_group_name = ::std::option::Option::Some(input.into());
        self
    }

    /// Test generation job group name
    pub fn set_test_generation_job_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_generation_job_group_name = input;
        self
    }

    /// Test generation job group name
    pub fn get_test_generation_job_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.test_generation_job_group_name
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn test_generation_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_generation_job_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_test_generation_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_generation_job_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_test_generation_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.test_generation_job_id
    }

    /// Consumes the builder and constructs a
    /// [`UnitTestGenerationExportContext`](crate::types::UnitTestGenerationExportContext). This
    /// method will fail if any of the following fields are not set:
    /// - [`test_generation_job_group_name`](crate::types::builders::UnitTestGenerationExportContextBuilder::test_generation_job_group_name)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::UnitTestGenerationExportContext,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::UnitTestGenerationExportContext {
            test_generation_job_group_name: self.test_generation_job_group_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "test_generation_job_group_name",
                    "test_generation_job_group_name was not specified but it is required when building UnitTestGenerationExportContext",
                )
            })?,
            test_generation_job_id: self.test_generation_job_id,
        })
    }
}