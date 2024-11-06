// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ResolutionSuggestion {
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub detailed_resolution_options: ::std::option::Option<::std::vec::Vec<crate::types::DetailedResolution>>,
}
impl ResolutionSuggestion {
    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.detailed_resolution_options.is_none()`.
    pub fn detailed_resolution_options(&self) -> &[crate::types::DetailedResolution] {
        self.detailed_resolution_options.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for ResolutionSuggestion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResolutionSuggestion");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("detailed_resolution_options", &self.detailed_resolution_options);
        formatter.finish()
    }
}
impl ResolutionSuggestion {
    /// Creates a new builder-style object to manufacture
    /// [`ResolutionSuggestion`](crate::types::ResolutionSuggestion).
    pub fn builder() -> crate::types::builders::ResolutionSuggestionBuilder {
        crate::types::builders::ResolutionSuggestionBuilder::default()
    }
}

/// A builder for [`ResolutionSuggestion`](crate::types::ResolutionSuggestion).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ResolutionSuggestionBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) detailed_resolution_options: ::std::option::Option<::std::vec::Vec<crate::types::DetailedResolution>>,
}
impl ResolutionSuggestionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }

    /// Appends an item to `detailed_resolution_options`.
    ///
    /// To override the contents of this collection use
    /// [`set_detailed_resolution_options`](Self::set_detailed_resolution_options).
    pub fn detailed_resolution_options(mut self, input: crate::types::DetailedResolution) -> Self {
        let mut v = self.detailed_resolution_options.unwrap_or_default();
        v.push(input);
        self.detailed_resolution_options = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_detailed_resolution_options(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DetailedResolution>>,
    ) -> Self {
        self.detailed_resolution_options = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_detailed_resolution_options(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::DetailedResolution>> {
        &self.detailed_resolution_options
    }

    /// Consumes the builder and constructs a
    /// [`ResolutionSuggestion`](crate::types::ResolutionSuggestion).
    pub fn build(self) -> crate::types::ResolutionSuggestion {
        crate::types::ResolutionSuggestion {
            description: self.description,
            detailed_resolution_options: self.detailed_resolution_options,
        }
    }
}
impl ::std::fmt::Debug for ResolutionSuggestionBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResolutionSuggestionBuilder");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("detailed_resolution_options", &self.detailed_resolution_options);
        formatter.finish()
    }
}