// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetCodeAnalysisOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub status: crate::types::CodeAnalysisStatus,
    #[allow(missing_docs)] // documentation missing in model
    pub error_message: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetCodeAnalysisOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> &crate::types::CodeAnalysisStatus {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn error_message(&self) -> ::std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
impl ::std::fmt::Debug for GetCodeAnalysisOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetCodeAnalysisOutput");
        formatter.field("status", &self.status);
        formatter.field("error_message", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for GetCodeAnalysisOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCodeAnalysisOutput {
    /// Creates a new builder-style object to manufacture
    /// [`GetCodeAnalysisOutput`](crate::operation::get_code_analysis::GetCodeAnalysisOutput).
    pub fn builder() -> crate::operation::get_code_analysis::builders::GetCodeAnalysisOutputBuilder {
        crate::operation::get_code_analysis::builders::GetCodeAnalysisOutputBuilder::default()
    }
}

/// A builder for
/// [`GetCodeAnalysisOutput`](crate::operation::get_code_analysis::GetCodeAnalysisOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GetCodeAnalysisOutputBuilder {
    pub(crate) status: ::std::option::Option<crate::types::CodeAnalysisStatus>,
    pub(crate) error_message: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetCodeAnalysisOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn status(mut self, input: crate::types::CodeAnalysisStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CodeAnalysisStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CodeAnalysisStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn error_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_message = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_error_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_message = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_error_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_message
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
    /// [`GetCodeAnalysisOutput`](crate::operation::get_code_analysis::GetCodeAnalysisOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](crate::operation::get_code_analysis::builders::GetCodeAnalysisOutputBuilder::status)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_code_analysis::GetCodeAnalysisOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_code_analysis::GetCodeAnalysisOutput {
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building GetCodeAnalysisOutput",
                )
            })?,
            error_message: self.error_message,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for GetCodeAnalysisOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetCodeAnalysisOutputBuilder");
        formatter.field("status", &self.status);
        formatter.field("error_message", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
