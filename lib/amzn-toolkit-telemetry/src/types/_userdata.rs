// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Userdata {
    #[allow(missing_docs)] // documentation missing in model
    pub email: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub comment: ::std::option::Option<::std::string::String>,
}
impl Userdata {
    #[allow(missing_docs)] // documentation missing in model
    pub fn email(&self) -> ::std::option::Option<&str> {
        self.email.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
}
impl Userdata {
    /// Creates a new builder-style object to manufacture [`Userdata`](crate::types::Userdata).
    pub fn builder() -> crate::types::builders::UserdataBuilder {
        crate::types::builders::UserdataBuilder::default()
    }
}

/// A builder for [`Userdata`](crate::types::Userdata).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UserdataBuilder {
    pub(crate) email: ::std::option::Option<::std::string::String>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
}
impl UserdataBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn email(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.email = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_email(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.email = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_email(&self) -> &::std::option::Option<::std::string::String> {
        &self.email
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        &self.comment
    }

    /// Consumes the builder and constructs a [`Userdata`](crate::types::Userdata).
    pub fn build(self) -> crate::types::Userdata {
        crate::types::Userdata {
            email: self.email,
            comment: self.comment,
        }
    }
}