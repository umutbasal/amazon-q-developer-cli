// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransformMetrics {
    #[allow(missing_docs)] // documentation missing in model
    pub transforms_performed: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub lines_of_code_changed: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub lines_of_code_submitted: ::std::option::Option<i64>,
}
impl TransformMetrics {
    #[allow(missing_docs)] // documentation missing in model
    pub fn transforms_performed(&self) -> ::std::option::Option<i64> {
        self.transforms_performed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_changed(&self) -> ::std::option::Option<i64> {
        self.lines_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_submitted(&self) -> ::std::option::Option<i64> {
        self.lines_of_code_submitted
    }
}
impl TransformMetrics {
    /// Creates a new builder-style object to manufacture
    /// [`TransformMetrics`](crate::types::TransformMetrics).
    pub fn builder() -> crate::types::builders::TransformMetricsBuilder {
        crate::types::builders::TransformMetricsBuilder::default()
    }
}

/// A builder for [`TransformMetrics`](crate::types::TransformMetrics).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransformMetricsBuilder {
    pub(crate) transforms_performed: ::std::option::Option<i64>,
    pub(crate) lines_of_code_changed: ::std::option::Option<i64>,
    pub(crate) lines_of_code_submitted: ::std::option::Option<i64>,
}
impl TransformMetricsBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn transforms_performed(mut self, input: i64) -> Self {
        self.transforms_performed = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_transforms_performed(mut self, input: ::std::option::Option<i64>) -> Self {
        self.transforms_performed = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_transforms_performed(&self) -> &::std::option::Option<i64> {
        &self.transforms_performed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_changed(mut self, input: i64) -> Self {
        self.lines_of_code_changed = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_lines_of_code_changed(mut self, input: ::std::option::Option<i64>) -> Self {
        self.lines_of_code_changed = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_lines_of_code_changed(&self) -> &::std::option::Option<i64> {
        &self.lines_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_submitted(mut self, input: i64) -> Self {
        self.lines_of_code_submitted = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_lines_of_code_submitted(mut self, input: ::std::option::Option<i64>) -> Self {
        self.lines_of_code_submitted = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_lines_of_code_submitted(&self) -> &::std::option::Option<i64> {
        &self.lines_of_code_submitted
    }

    /// Consumes the builder and constructs a [`TransformMetrics`](crate::types::TransformMetrics).
    pub fn build(self) -> crate::types::TransformMetrics {
        crate::types::TransformMetrics {
            transforms_performed: self.transforms_performed,
            lines_of_code_changed: self.lines_of_code_changed,
            lines_of_code_submitted: self.lines_of_code_submitted,
        }
    }
}