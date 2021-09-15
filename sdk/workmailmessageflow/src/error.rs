// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetRawMessageContentError {
    pub kind: GetRawMessageContentErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetRawMessageContentErrorKind {
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetRawMessageContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRawMessageContentErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetRawMessageContentErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for GetRawMessageContentError {
    fn code(&self) -> Option<&str> {
        GetRawMessageContentError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRawMessageContentError {
    pub fn new(kind: GetRawMessageContentErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetRawMessageContentErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetRawMessageContentErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetRawMessageContentErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for GetRawMessageContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRawMessageContentErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetRawMessageContentErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutRawMessageContentError {
    pub kind: PutRawMessageContentErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutRawMessageContentErrorKind {
    InvalidContentLocation(crate::error::InvalidContentLocation),
    MessageFrozen(crate::error::MessageFrozen),
    MessageRejected(crate::error::MessageRejected),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for PutRawMessageContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutRawMessageContentErrorKind::InvalidContentLocation(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::MessageFrozen(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::MessageRejected(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for PutRawMessageContentError {
    fn code(&self) -> Option<&str> {
        PutRawMessageContentError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutRawMessageContentError {
    pub fn new(kind: PutRawMessageContentErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutRawMessageContentErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutRawMessageContentErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_invalid_content_location(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::InvalidContentLocation(_)
        )
    }
    pub fn is_message_frozen(&self) -> bool {
        matches!(&self.kind, PutRawMessageContentErrorKind::MessageFrozen(_))
    }
    pub fn is_message_rejected(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::MessageRejected(_)
        )
    }
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for PutRawMessageContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutRawMessageContentErrorKind::InvalidContentLocation(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::MessageFrozen(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::MessageRejected(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The requested email message is not found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>The requested email could not be updated due to an error in the MIME content. Check the error message for more information about
/// what caused the error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MessageRejected {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for MessageRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MessageRejected");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl MessageRejected {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MessageRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageRejected")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for MessageRejected {}
/// See [`MessageRejected`](crate::error::MessageRejected)
pub mod message_rejected {
    /// A builder for [`MessageRejected`](crate::error::MessageRejected)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageRejected`](crate::error::MessageRejected)
        pub fn build(self) -> crate::error::MessageRejected {
            crate::error::MessageRejected {
                message: self.message,
            }
        }
    }
}
impl MessageRejected {
    /// Creates a new builder-style object to manufacture [`MessageRejected`](crate::error::MessageRejected)
    pub fn builder() -> crate::error::message_rejected::Builder {
        crate::error::message_rejected::Builder::default()
    }
}

/// <p>The requested email is not eligible for update. This is usually the case for a redirected email.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MessageFrozen {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for MessageFrozen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MessageFrozen");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl MessageFrozen {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MessageFrozen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageFrozen")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for MessageFrozen {}
/// See [`MessageFrozen`](crate::error::MessageFrozen)
pub mod message_frozen {
    /// A builder for [`MessageFrozen`](crate::error::MessageFrozen)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageFrozen`](crate::error::MessageFrozen)
        pub fn build(self) -> crate::error::MessageFrozen {
            crate::error::MessageFrozen {
                message: self.message,
            }
        }
    }
}
impl MessageFrozen {
    /// Creates a new builder-style object to manufacture [`MessageFrozen`](crate::error::MessageFrozen)
    pub fn builder() -> crate::error::message_frozen::Builder {
        crate::error::message_frozen::Builder::default()
    }
}

/// <p>WorkMail could not access the updated email content. Possible reasons:</p>
/// <ul>
/// <li>
/// <p>You made the request in a region other than your S3 bucket region.</p>
/// </li>
/// <li>
/// <p>The <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-owner-condition.html">S3 bucket owner</a> is not the
/// same as the calling AWS account.</p>
/// </li>
/// <li>
/// <p>You have an incomplete or missing S3 bucket policy. For more information about policies, see
/// <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html">
/// Updating message content with AWS Lambda
/// </a> in the <i>WorkMail Administrator
/// Guide</i>.</p>
/// </li>
/// </ul>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidContentLocation {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidContentLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidContentLocation");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidContentLocation {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidContentLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidContentLocation")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidContentLocation {}
/// See [`InvalidContentLocation`](crate::error::InvalidContentLocation)
pub mod invalid_content_location {
    /// A builder for [`InvalidContentLocation`](crate::error::InvalidContentLocation)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidContentLocation`](crate::error::InvalidContentLocation)
        pub fn build(self) -> crate::error::InvalidContentLocation {
            crate::error::InvalidContentLocation {
                message: self.message,
            }
        }
    }
}
impl InvalidContentLocation {
    /// Creates a new builder-style object to manufacture [`InvalidContentLocation`](crate::error::InvalidContentLocation)
    pub fn builder() -> crate::error::invalid_content_location::Builder {
        crate::error::invalid_content_location::Builder::default()
    }
}