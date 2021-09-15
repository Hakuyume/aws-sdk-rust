// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AmazonAppIntegrationService`.
///
/// This client allows ergonomic access to a `AmazonAppIntegrationService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn create_event_integration(&self) -> fluent_builders::CreateEventIntegration<C, M, R> {
        fluent_builders::CreateEventIntegration::new(self.handle.clone())
    }
    pub fn delete_event_integration(&self) -> fluent_builders::DeleteEventIntegration<C, M, R> {
        fluent_builders::DeleteEventIntegration::new(self.handle.clone())
    }
    pub fn get_event_integration(&self) -> fluent_builders::GetEventIntegration<C, M, R> {
        fluent_builders::GetEventIntegration::new(self.handle.clone())
    }
    pub fn list_event_integration_associations(
        &self,
    ) -> fluent_builders::ListEventIntegrationAssociations<C, M, R> {
        fluent_builders::ListEventIntegrationAssociations::new(self.handle.clone())
    }
    pub fn list_event_integrations(&self) -> fluent_builders::ListEventIntegrations<C, M, R> {
        fluent_builders::ListEventIntegrations::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C, M, R> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C, M, R> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C, M, R> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    pub fn update_event_integration(&self) -> fluent_builders::UpdateEventIntegration<C, M, R> {
        fluent_builders::UpdateEventIntegration::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateEventIntegration<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_event_integration_input::Builder,
    }
    impl<C, M, R> CreateEventIntegration<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateEventIntegrationOutput,
            smithy_http::result::SdkError<crate::error::CreateEventIntegrationError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateEventIntegrationInputOperationOutputAlias,
                crate::output::CreateEventIntegrationOutput,
                crate::error::CreateEventIntegrationError,
                crate::input::CreateEventIntegrationInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the event integration.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>The description of the event integration.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The event filter.</p>
        pub fn event_filter(mut self, inp: crate::model::EventFilter) -> Self {
            self.inner = self.inner.event_filter(inp);
            self
        }
        pub fn set_event_filter(
            mut self,
            input: std::option::Option<crate::model::EventFilter>,
        ) -> Self {
            self.inner = self.inner.set_event_filter(input);
            self
        }
        /// <p>The EventBridge bus.</p>
        pub fn event_bridge_bus(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_bridge_bus(inp);
            self
        }
        pub fn set_event_bridge_bus(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_bridge_bus(input);
            self
        }
        /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
        /// request.</p>
        pub fn client_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_token(inp);
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_token(input);
            self
        }
        /// Adds a key-value pair to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>One or more tags.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteEventIntegration<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_event_integration_input::Builder,
    }
    impl<C, M, R> DeleteEventIntegration<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteEventIntegrationOutput,
            smithy_http::result::SdkError<crate::error::DeleteEventIntegrationError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteEventIntegrationInputOperationOutputAlias,
                crate::output::DeleteEventIntegrationOutput,
                crate::error::DeleteEventIntegrationError,
                crate::input::DeleteEventIntegrationInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the event integration.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetEventIntegration<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_event_integration_input::Builder,
    }
    impl<C, M, R> GetEventIntegration<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetEventIntegrationOutput,
            smithy_http::result::SdkError<crate::error::GetEventIntegrationError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetEventIntegrationInputOperationOutputAlias,
                crate::output::GetEventIntegrationOutput,
                crate::error::GetEventIntegrationError,
                crate::input::GetEventIntegrationInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the event integration. </p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListEventIntegrationAssociations<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_event_integration_associations_input::Builder,
    }
    impl<C, M, R> ListEventIntegrationAssociations<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListEventIntegrationAssociationsOutput,
            smithy_http::result::SdkError<crate::error::ListEventIntegrationAssociationsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListEventIntegrationAssociationsInputOperationOutputAlias,
                crate::output::ListEventIntegrationAssociationsOutput,
                crate::error::ListEventIntegrationAssociationsError,
                crate::input::ListEventIntegrationAssociationsInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the event integration. </p>
        pub fn event_integration_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_integration_name(inp);
            self
        }
        pub fn set_event_integration_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_integration_name(input);
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return per page.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListEventIntegrations<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_event_integrations_input::Builder,
    }
    impl<C, M, R> ListEventIntegrations<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListEventIntegrationsOutput,
            smithy_http::result::SdkError<crate::error::ListEventIntegrationsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListEventIntegrationsInputOperationOutputAlias,
                crate::output::ListEventIntegrationsOutput,
                crate::error::ListEventIntegrationsError,
                crate::input::ListEventIntegrationsInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return per page.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C, M, R> ListTagsForResource<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListTagsForResourceOutput,
            smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListTagsForResourceInputOperationOutputAlias,
                crate::output::ListTagsForResourceOutput,
                crate::error::ListTagsForResourceError,
                crate::input::ListTagsForResourceInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource. </p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C, M, R> TagResource<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::TagResourceOutput,
            smithy_http::result::SdkError<crate::error::TagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::TagResourceInputOperationOutputAlias,
                crate::output::TagResourceOutput,
                crate::error::TagResourceError,
                crate::input::TagResourceInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>One or more tags. </p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UntagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C, M, R> UntagResource<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UntagResourceOutput,
            smithy_http::result::SdkError<crate::error::UntagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UntagResourceInputOperationOutputAlias,
                crate::output::UntagResourceOutput,
                crate::error::UntagResourceError,
                crate::input::UntagResourceInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Appends an item to `tagKeys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        /// <p>The tag keys.</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateEventIntegration<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_event_integration_input::Builder,
    }
    impl<C, M, R> UpdateEventIntegration<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateEventIntegrationOutput,
            smithy_http::result::SdkError<crate::error::UpdateEventIntegrationError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateEventIntegrationInputOperationOutputAlias,
                crate::output::UpdateEventIntegrationOutput,
                crate::error::UpdateEventIntegrationError,
                crate::input::UpdateEventIntegrationInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the event integration.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>The description of the event inegration.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}