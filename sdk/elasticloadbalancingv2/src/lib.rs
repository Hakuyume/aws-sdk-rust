#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <fullname>Elastic Load Balancing</fullname>
//! <p>A load balancer distributes incoming traffic across targets, such as your EC2 instances.
//! This enables you to increase the availability of your application. The load balancer also
//! monitors the health of its registered targets and ensures that it routes traffic only to
//! healthy targets. You configure your load balancer to accept incoming traffic by specifying one
//! or more listeners, which are configured with a protocol and port number for connections from
//! clients to the load balancer. You configure a target group with a protocol and port number for
//! connections from the load balancer to the targets, and with health check settings to be used
//! when checking the health status of the targets.</p>
//! <p>Elastic Load Balancing supports the following types of load balancers: Application Load
//! Balancers, Network Load Balancers, Gateway Load Balancers, and Classic Load Balancers. This
//! reference covers the following load balancer types:</p>
//! <ul>
//! <li>
//! <p>Application Load Balancer - Operates at the application layer (layer 7) and supports
//! HTTP and HTTPS.</p>
//! </li>
//! <li>
//! <p>Network Load Balancer - Operates at the transport layer (layer 4) and supports TCP,
//! TLS, and UDP.</p>
//! </li>
//! <li>
//! <p>Gateway Load Balancer - Operates at the network layer (layer 3).</p>
//! </li>
//! </ul>
//! <p>For more information, see the <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/">Elastic Load Balancing User
//! Guide</a>.</p>
//! <p>All Elastic Load Balancing operations are idempotent, which means that they complete at
//! most one time. If you repeat an operation, it succeeds.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("elasticloadbalancingv2", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;