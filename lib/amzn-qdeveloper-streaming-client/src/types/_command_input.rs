// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// CommandInput can be extended to either a list of strings or a single string.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum CommandInput {
    /// The list of context items used to generate output.
    CommandsList(::std::vec::Vec<::std::string::String>),
    /// The `Unknown` variant represents cases where new union variant was received. Consider
    /// upgrading the SDK to the latest available version. An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has
    /// not been updated. To investigate this, consider turning on debug logging to print the
    /// raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl CommandInput {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into
    /// [`CommandsList`](crate::types::CommandInput::CommandsList), extracting the inner
    /// [`Vec`](::std::vec::Vec). Returns `Err(&Self)` if it can't be converted.
    pub fn as_commands_list(&self) -> ::std::result::Result<&::std::vec::Vec<::std::string::String>, &Self> {
        if let CommandInput::CommandsList(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }

    /// Returns true if this is a [`CommandsList`](crate::types::CommandInput::CommandsList).
    pub fn is_commands_list(&self) -> bool {
        self.as_commands_list().is_ok()
    }

    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}