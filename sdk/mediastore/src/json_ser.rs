// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_container_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateContainerInput,
) {
    if let Some(var_1) = &input.container_name {
        object.key("ContainerName").string(var_1);
    }
    if let Some(var_2) = &input.tags {
        let mut array_3 = object.key("Tags").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4);
                object_5.finish();
            }
        }
        array_3.finish();
    }
}

pub fn serialize_structure_crate_input_delete_container_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteContainerInput,
) {
    if let Some(var_6) = &input.container_name {
        object.key("ContainerName").string(var_6);
    }
}

pub fn serialize_structure_crate_input_delete_container_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteContainerPolicyInput,
) {
    if let Some(var_7) = &input.container_name {
        object.key("ContainerName").string(var_7);
    }
}

pub fn serialize_structure_crate_input_delete_cors_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCorsPolicyInput,
) {
    if let Some(var_8) = &input.container_name {
        object.key("ContainerName").string(var_8);
    }
}

pub fn serialize_structure_crate_input_delete_lifecycle_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLifecyclePolicyInput,
) {
    if let Some(var_9) = &input.container_name {
        object.key("ContainerName").string(var_9);
    }
}

pub fn serialize_structure_crate_input_delete_metric_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMetricPolicyInput,
) {
    if let Some(var_10) = &input.container_name {
        object.key("ContainerName").string(var_10);
    }
}

pub fn serialize_structure_crate_input_describe_container_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeContainerInput,
) {
    if let Some(var_11) = &input.container_name {
        object.key("ContainerName").string(var_11);
    }
}

pub fn serialize_structure_crate_input_get_container_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetContainerPolicyInput,
) {
    if let Some(var_12) = &input.container_name {
        object.key("ContainerName").string(var_12);
    }
}

pub fn serialize_structure_crate_input_get_cors_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCorsPolicyInput,
) {
    if let Some(var_13) = &input.container_name {
        object.key("ContainerName").string(var_13);
    }
}

pub fn serialize_structure_crate_input_get_lifecycle_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLifecyclePolicyInput,
) {
    if let Some(var_14) = &input.container_name {
        object.key("ContainerName").string(var_14);
    }
}

pub fn serialize_structure_crate_input_get_metric_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMetricPolicyInput,
) {
    if let Some(var_15) = &input.container_name {
        object.key("ContainerName").string(var_15);
    }
}

pub fn serialize_structure_crate_input_list_containers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListContainersInput,
) {
    if let Some(var_16) = &input.next_token {
        object.key("NextToken").string(var_16);
    }
    if let Some(var_17) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_17).into()),
        );
    }
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_18) = &input.resource {
        object.key("Resource").string(var_18);
    }
}

pub fn serialize_structure_crate_input_put_container_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutContainerPolicyInput,
) {
    if let Some(var_19) = &input.container_name {
        object.key("ContainerName").string(var_19);
    }
    if let Some(var_20) = &input.policy {
        object.key("Policy").string(var_20);
    }
}

pub fn serialize_structure_crate_input_put_cors_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutCorsPolicyInput,
) {
    if let Some(var_21) = &input.container_name {
        object.key("ContainerName").string(var_21);
    }
    if let Some(var_22) = &input.cors_policy {
        let mut array_23 = object.key("CorsPolicy").start_array();
        for item_24 in var_22 {
            {
                let mut object_25 = array_23.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cors_rule(&mut object_25, item_24);
                object_25.finish();
            }
        }
        array_23.finish();
    }
}

pub fn serialize_structure_crate_input_put_lifecycle_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutLifecyclePolicyInput,
) {
    if let Some(var_26) = &input.container_name {
        object.key("ContainerName").string(var_26);
    }
    if let Some(var_27) = &input.lifecycle_policy {
        object.key("LifecyclePolicy").string(var_27);
    }
}

pub fn serialize_structure_crate_input_put_metric_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMetricPolicyInput,
) {
    if let Some(var_28) = &input.container_name {
        object.key("ContainerName").string(var_28);
    }
    if let Some(var_29) = &input.metric_policy {
        let mut object_30 = object.key("MetricPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_metric_policy(&mut object_30, var_29);
        object_30.finish();
    }
}

pub fn serialize_structure_crate_input_start_access_logging_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartAccessLoggingInput,
) {
    if let Some(var_31) = &input.container_name {
        object.key("ContainerName").string(var_31);
    }
}

pub fn serialize_structure_crate_input_stop_access_logging_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopAccessLoggingInput,
) {
    if let Some(var_32) = &input.container_name {
        object.key("ContainerName").string(var_32);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_33) = &input.resource {
        object.key("Resource").string(var_33);
    }
    if let Some(var_34) = &input.tags {
        let mut array_35 = object.key("Tags").start_array();
        for item_36 in var_34 {
            {
                let mut object_37 = array_35.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_37, item_36);
                object_37.finish();
            }
        }
        array_35.finish();
    }
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_38) = &input.resource {
        object.key("Resource").string(var_38);
    }
    if let Some(var_39) = &input.tag_keys {
        let mut array_40 = object.key("TagKeys").start_array();
        for item_41 in var_39 {
            {
                array_40.value().string(item_41);
            }
        }
        array_40.finish();
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_42) = &input.key {
        object.key("Key").string(var_42);
    }
    if let Some(var_43) = &input.value {
        object.key("Value").string(var_43);
    }
}

pub fn serialize_structure_crate_model_cors_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CorsRule,
) {
    if let Some(var_44) = &input.allowed_origins {
        let mut array_45 = object.key("AllowedOrigins").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46);
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.allowed_methods {
        let mut array_48 = object.key("AllowedMethods").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.allowed_headers {
        let mut array_51 = object.key("AllowedHeaders").start_array();
        for item_52 in var_50 {
            {
                array_51.value().string(item_52);
            }
        }
        array_51.finish();
    }
    if input.max_age_seconds != 0 {
        object.key("MaxAgeSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_age_seconds).into()),
        );
    }
    if let Some(var_53) = &input.expose_headers {
        let mut array_54 = object.key("ExposeHeaders").start_array();
        for item_55 in var_53 {
            {
                array_54.value().string(item_55);
            }
        }
        array_54.finish();
    }
}

pub fn serialize_structure_crate_model_metric_policy(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricPolicy,
) {
    if let Some(var_56) = &input.container_level_metrics {
        object.key("ContainerLevelMetrics").string(var_56.as_str());
    }
    if let Some(var_57) = &input.metric_policy_rules {
        let mut array_58 = object.key("MetricPolicyRules").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_policy_rule(
                    &mut object_60,
                    item_59,
                );
                object_60.finish();
            }
        }
        array_58.finish();
    }
}

pub fn serialize_structure_crate_model_metric_policy_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricPolicyRule,
) {
    if let Some(var_61) = &input.object_group {
        object.key("ObjectGroup").string(var_61);
    }
    if let Some(var_62) = &input.object_group_name {
        object.key("ObjectGroupName").string(var_62);
    }
}