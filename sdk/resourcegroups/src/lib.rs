#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <fullname>AWS Resource Groups</fullname>
//! <p>AWS Resource Groups lets you organize AWS resources such as Amazon EC2 instances, Amazon Relational Database Service
//! databases, and Amazon S3 buckets into groups using criteria that you define as tags. A
//! resource group is a collection of resources that match the resource types specified in a
//! query, and share one or more tags or portions of tags. You can create a group of
//! resources based on their roles in your cloud infrastructure, lifecycle stages, regions,
//! application layers, or virtually any criteria. Resource Groups enable you to automate management
//! tasks, such as those in AWS Systems Manager Automation documents, on tag-related resources in
//! AWS Systems Manager. Groups of tagged resources also let you quickly view a custom console in
//! AWS Systems Manager that shows AWS Config compliance and other monitoring data about member
//! resources.</p>
//! <p>To create a resource group, build a resource query, and specify tags that identify the
//! criteria that members of the group have in common. Tags are key-value pairs.</p>
//! <p>For more information about Resource Groups, see the <a href="https://docs.aws.amazon.com/ARG/latest/userguide/welcome.html">AWS Resource Groups User Guide</a>.</p>
//! <p>AWS Resource Groups uses a REST-compliant API that you can use to perform the following types of
//! operations.</p>
//! <ul>
//! <li>
//! <p>Create, Read, Update, and Delete (CRUD) operations on resource groups and
//! resource query entities</p>
//! </li>
//! <li>
//! <p>Applying, editing, and removing tags from resource groups</p>
//! </li>
//! <li>
//! <p>Resolving resource group member ARNs so they can be returned as search
//! results</p>
//! </li>
//! <li>
//! <p>Getting data about resources that are members of a group</p>
//! </li>
//! <li>
//! <p>Searching AWS resources based on a resource query</p>
//! </li>
//! </ul>

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
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("resourcegroups", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;