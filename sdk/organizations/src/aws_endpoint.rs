// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
            .id("aws")
            .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
            .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                uri_template: "organizations.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope: aws_endpoint::CredentialScope::builder().build(),
            })
            .partition_endpoint("aws-global")
            .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
            .endpoint(
                "aws-global",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations.us-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .endpoint(
                "fips-aws-global",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations-fips.us-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .build()
            .expect("invalid partition"),
        vec![
            aws_endpoint::Partition::builder()
                .id("aws-cn")
                .region_regex(r#"^cn\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-cn-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-cn-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "organizations.cn-northwest-1.amazonaws.com.cn",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("cn-northwest-1")
                            .build(),
                    },
                )
                .endpoint(
                    "fips-aws-cn-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "organizations.cn-northwest-1.amazonaws.com.cn",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("cn-northwest-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso")
                .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations.{region}.c2s.ic.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso-b")
                .region_regex(r#"^us\-isob\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations.{region}.sc2s.sgov.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-us-gov")
                .region_regex(r#"^us\-gov\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "organizations.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-us-gov-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-us-gov-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "organizations.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .endpoint(
                    "fips-aws-us-gov-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "organizations.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
        ],
    )
}