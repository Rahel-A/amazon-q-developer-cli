// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UsePluginInput {
    #[allow(missing_docs)] // documentation missing in model
    pub plugin_arn: ::std::option::Option<::std::string::String>,
}
impl UsePluginInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn plugin_arn(&self) -> ::std::option::Option<&str> {
        self.plugin_arn.as_deref()
    }
}
impl UsePluginInput {
    /// Creates a new builder-style object to manufacture
    /// [`UsePluginInput`](crate::operation::use_plugin::UsePluginInput).
    pub fn builder() -> crate::operation::use_plugin::builders::UsePluginInputBuilder {
        crate::operation::use_plugin::builders::UsePluginInputBuilder::default()
    }
}

/// A builder for [`UsePluginInput`](crate::operation::use_plugin::UsePluginInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UsePluginInputBuilder {
    pub(crate) plugin_arn: ::std::option::Option<::std::string::String>,
}
impl UsePluginInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn plugin_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.plugin_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_plugin_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.plugin_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_plugin_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.plugin_arn
    }

    /// Consumes the builder and constructs a
    /// [`UsePluginInput`](crate::operation::use_plugin::UsePluginInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::use_plugin::UsePluginInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::use_plugin::UsePluginInput {
            plugin_arn: self.plugin_arn,
        })
    }
}
