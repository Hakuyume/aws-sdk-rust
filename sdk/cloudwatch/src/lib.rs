#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>Amazon CloudWatch monitors your Amazon Web Services (Amazon Web Services) resources and the
//! applications you run on Amazon Web Services in real time. You can use CloudWatch to collect and track
//! metrics, which are the variables you want to measure for your resources and
//! applications.</p>
//! <p>CloudWatch alarms send notifications or automatically change the resources you are monitoring based on rules
//! that you define. For example, you can monitor the CPU usage and disk reads and writes of your Amazon EC2
//! instances. Then, use this data to determine whether you should launch
//! additional instances to handle increased load. You can also use this data to stop
//! under-used instances to save
//! money.</p>
//! <p>In addition to monitoring the built-in metrics that come with Amazon Web Services, you can monitor
//! your own custom metrics. With CloudWatch, you gain system-wide visibility into resource
//! utilization, application performance, and operational health.</p>

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
    aws_http::user_agent::ApiMetadata::new("cloudwatch", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;