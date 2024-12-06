// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SuggestedFix {
    #[allow(missing_docs)] // documentation missing in model
    pub code_diff: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
}
impl SuggestedFix {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code_diff(&self) -> ::std::option::Option<&str> {
        self.code_diff.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.references.is_none()`.
    pub fn references(&self) -> &[crate::types::Reference] {
        self.references.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for SuggestedFix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SuggestedFix");
        formatter.field("code_diff", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.finish()
    }
}
impl SuggestedFix {
    /// Creates a new builder-style object to manufacture
    /// [`SuggestedFix`](crate::types::SuggestedFix).
    pub fn builder() -> crate::types::builders::SuggestedFixBuilder {
        crate::types::builders::SuggestedFixBuilder::default()
    }
}

/// A builder for [`SuggestedFix`](crate::types::SuggestedFix).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SuggestedFixBuilder {
    pub(crate) code_diff: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
}
impl SuggestedFixBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code_diff(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code_diff = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_code_diff(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code_diff = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_code_diff(&self) -> &::std::option::Option<::std::string::String> {
        &self.code_diff
    }

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

    /// Appends an item to `references`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    pub fn references(mut self, input: crate::types::Reference) -> Self {
        let mut v = self.references.unwrap_or_default();
        v.push(input);
        self.references = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_references(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>) -> Self {
        self.references = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_references(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Reference>> {
        &self.references
    }

    /// Consumes the builder and constructs a [`SuggestedFix`](crate::types::SuggestedFix).
    pub fn build(self) -> crate::types::SuggestedFix {
        crate::types::SuggestedFix {
            code_diff: self.code_diff,
            description: self.description,
            references: self.references,
        }
    }
}
impl ::std::fmt::Debug for SuggestedFixBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SuggestedFixBuilder");
        formatter.field("code_diff", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.finish()
    }
}