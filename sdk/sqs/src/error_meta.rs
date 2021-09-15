// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BatchEntryIdsNotDistinct(crate::error::BatchEntryIdsNotDistinct),
    BatchRequestTooLong(crate::error::BatchRequestTooLong),
    EmptyBatchRequest(crate::error::EmptyBatchRequest),
    InvalidAttributeName(crate::error::InvalidAttributeName),
    InvalidBatchEntryId(crate::error::InvalidBatchEntryId),
    InvalidIdFormat(crate::error::InvalidIdFormat),
    InvalidMessageContents(crate::error::InvalidMessageContents),
    MessageNotInflight(crate::error::MessageNotInflight),
    OverLimit(crate::error::OverLimit),
    PurgeQueueInProgress(crate::error::PurgeQueueInProgress),
    QueueDeletedRecently(crate::error::QueueDeletedRecently),
    QueueDoesNotExist(crate::error::QueueDoesNotExist),
    QueueNameExists(crate::error::QueueNameExists),
    ReceiptHandleIsInvalid(crate::error::ReceiptHandleIsInvalid),
    TooManyEntriesInBatchRequest(crate::error::TooManyEntriesInBatchRequest),
    UnsupportedOperation(crate::error::UnsupportedOperation),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BatchEntryIdsNotDistinct(inner) => inner.fmt(f),
            Error::BatchRequestTooLong(inner) => inner.fmt(f),
            Error::EmptyBatchRequest(inner) => inner.fmt(f),
            Error::InvalidAttributeName(inner) => inner.fmt(f),
            Error::InvalidBatchEntryId(inner) => inner.fmt(f),
            Error::InvalidIdFormat(inner) => inner.fmt(f),
            Error::InvalidMessageContents(inner) => inner.fmt(f),
            Error::MessageNotInflight(inner) => inner.fmt(f),
            Error::OverLimit(inner) => inner.fmt(f),
            Error::PurgeQueueInProgress(inner) => inner.fmt(f),
            Error::QueueDeletedRecently(inner) => inner.fmt(f),
            Error::QueueDoesNotExist(inner) => inner.fmt(f),
            Error::QueueNameExists(inner) => inner.fmt(f),
            Error::ReceiptHandleIsInvalid(inner) => inner.fmt(f),
            Error::TooManyEntriesInBatchRequest(inner) => inner.fmt(f),
            Error::UnsupportedOperation(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AddPermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::AddPermissionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AddPermissionErrorKind::OverLimit(inner) => Error::OverLimit(inner),
                crate::error::AddPermissionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ChangeMessageVisibilityErrorKind::MessageNotInflight(inner) => {
                    Error::MessageNotInflight(inner)
                }
                crate::error::ChangeMessageVisibilityErrorKind::ReceiptHandleIsInvalid(inner) => {
                    Error::ReceiptHandleIsInvalid(inner)
                }
                crate::error::ChangeMessageVisibilityErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityBatchError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityBatchError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ChangeMessageVisibilityBatchErrorKind::BatchEntryIdsNotDistinct(inner) => Error::BatchEntryIdsNotDistinct(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::EmptyBatchRequest(inner) => Error::EmptyBatchRequest(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::InvalidBatchEntryId(inner) => Error::InvalidBatchEntryId(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::TooManyEntriesInBatchRequest(inner) => Error::TooManyEntriesInBatchRequest(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateQueueError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateQueueErrorKind::QueueDeletedRecently(inner) => {
                    Error::QueueDeletedRecently(inner)
                }
                crate::error::CreateQueueErrorKind::QueueNameExists(inner) => {
                    Error::QueueNameExists(inner)
                }
                crate::error::CreateQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteMessageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteMessageErrorKind::InvalidIdFormat(inner) => {
                    Error::InvalidIdFormat(inner)
                }
                crate::error::DeleteMessageErrorKind::ReceiptHandleIsInvalid(inner) => {
                    Error::ReceiptHandleIsInvalid(inner)
                }
                crate::error::DeleteMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteMessageBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteMessageBatchError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteMessageBatchErrorKind::BatchEntryIdsNotDistinct(inner) => {
                    Error::BatchEntryIdsNotDistinct(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::EmptyBatchRequest(inner) => {
                    Error::EmptyBatchRequest(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::InvalidBatchEntryId(inner) => {
                    Error::InvalidBatchEntryId(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::TooManyEntriesInBatchRequest(inner) => {
                    Error::TooManyEntriesInBatchRequest(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteQueueError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetQueueAttributesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetQueueAttributesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQueueAttributesErrorKind::InvalidAttributeName(inner) => {
                    Error::InvalidAttributeName(inner)
                }
                crate::error::GetQueueAttributesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetQueueUrlError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetQueueUrlError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQueueUrlErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::GetQueueUrlErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListDeadLetterSourceQueuesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListDeadLetterSourceQueuesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDeadLetterSourceQueuesErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::ListDeadLetterSourceQueuesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListQueuesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListQueuesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQueuesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListQueueTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListQueueTagsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQueueTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PurgeQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::PurgeQueueError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PurgeQueueErrorKind::PurgeQueueInProgress(inner) => {
                    Error::PurgeQueueInProgress(inner)
                }
                crate::error::PurgeQueueErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::PurgeQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ReceiveMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ReceiveMessageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ReceiveMessageErrorKind::OverLimit(inner) => Error::OverLimit(inner),
                crate::error::ReceiveMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::RemovePermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::RemovePermissionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemovePermissionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SendMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::SendMessageError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendMessageErrorKind::InvalidMessageContents(inner) => {
                    Error::InvalidMessageContents(inner)
                }
                crate::error::SendMessageErrorKind::UnsupportedOperation(inner) => {
                    Error::UnsupportedOperation(inner)
                }
                crate::error::SendMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SendMessageBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::SendMessageBatchError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendMessageBatchErrorKind::BatchEntryIdsNotDistinct(inner) => {
                    Error::BatchEntryIdsNotDistinct(inner)
                }
                crate::error::SendMessageBatchErrorKind::BatchRequestTooLong(inner) => {
                    Error::BatchRequestTooLong(inner)
                }
                crate::error::SendMessageBatchErrorKind::EmptyBatchRequest(inner) => {
                    Error::EmptyBatchRequest(inner)
                }
                crate::error::SendMessageBatchErrorKind::InvalidBatchEntryId(inner) => {
                    Error::InvalidBatchEntryId(inner)
                }
                crate::error::SendMessageBatchErrorKind::TooManyEntriesInBatchRequest(inner) => {
                    Error::TooManyEntriesInBatchRequest(inner)
                }
                crate::error::SendMessageBatchErrorKind::UnsupportedOperation(inner) => {
                    Error::UnsupportedOperation(inner)
                }
                crate::error::SendMessageBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::SetQueueAttributesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::SetQueueAttributesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetQueueAttributesErrorKind::InvalidAttributeName(inner) => {
                    Error::InvalidAttributeName(inner)
                }
                crate::error::SetQueueAttributesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagQueueError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagQueueError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}