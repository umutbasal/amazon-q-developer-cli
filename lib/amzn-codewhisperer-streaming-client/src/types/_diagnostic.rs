// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents a Diagnostic message
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum Diagnostic {
    /// Diagnostics originating from a Runtime
    RuntimeDiagnostic(crate::types::RuntimeDiagnostic),
    /// Diagnostics originating from a TextDocument
    TextDocumentDiagnostic(crate::types::TextDocumentDiagnostic),
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
impl Diagnostic {
    /// Tries to convert the enum instance into
    /// [`RuntimeDiagnostic`](crate::types::Diagnostic::RuntimeDiagnostic), extracting the inner
    /// [`RuntimeDiagnostic`](crate::types::RuntimeDiagnostic). Returns `Err(&Self)` if it can't
    /// be converted.
    pub fn as_runtime_diagnostic(&self) -> ::std::result::Result<&crate::types::RuntimeDiagnostic, &Self> {
        if let Diagnostic::RuntimeDiagnostic(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }

    /// Returns true if this is a
    /// [`RuntimeDiagnostic`](crate::types::Diagnostic::RuntimeDiagnostic).
    pub fn is_runtime_diagnostic(&self) -> bool {
        self.as_runtime_diagnostic().is_ok()
    }

    /// Tries to convert the enum instance into
    /// [`TextDocumentDiagnostic`](crate::types::Diagnostic::TextDocumentDiagnostic), extracting the
    /// inner [`TextDocumentDiagnostic`](crate::types::TextDocumentDiagnostic). Returns `Err(&
    /// Self)` if it can't be converted.
    pub fn as_text_document_diagnostic(&self) -> ::std::result::Result<&crate::types::TextDocumentDiagnostic, &Self> {
        if let Diagnostic::TextDocumentDiagnostic(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }

    /// Returns true if this is a
    /// [`TextDocumentDiagnostic`](crate::types::Diagnostic::TextDocumentDiagnostic).
    pub fn is_text_document_diagnostic(&self) -> bool {
        self.as_text_document_diagnostic().is_ok()
    }

    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}