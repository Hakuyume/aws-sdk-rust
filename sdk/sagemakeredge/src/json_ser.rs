// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_device_registration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDeviceRegistrationInput,
) {
    if let Some(var_1) = &input.device_fleet_name {
        object.key("DeviceFleetName").string(var_1);
    }
    if let Some(var_2) = &input.device_name {
        object.key("DeviceName").string(var_2);
    }
}

pub fn serialize_structure_crate_input_send_heartbeat_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendHeartbeatInput,
) {
    if let Some(var_3) = &input.agent_metrics {
        let mut array_4 = object.key("AgentMetrics").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_edge_metric(&mut object_6, item_5);
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.agent_version {
        object.key("AgentVersion").string(var_7);
    }
    if let Some(var_8) = &input.device_fleet_name {
        object.key("DeviceFleetName").string(var_8);
    }
    if let Some(var_9) = &input.device_name {
        object.key("DeviceName").string(var_9);
    }
    if let Some(var_10) = &input.models {
        let mut array_11 = object.key("Models").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_model(&mut object_13, item_12);
                object_13.finish();
            }
        }
        array_11.finish();
    }
}

pub fn serialize_structure_crate_model_edge_metric(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EdgeMetric,
) {
    if let Some(var_14) = &input.dimension {
        object.key("Dimension").string(var_14);
    }
    if let Some(var_15) = &input.metric_name {
        object.key("MetricName").string(var_15);
    }
    if input.value != 0.0 {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.value).into()),
        );
    }
    if let Some(var_16) = &input.timestamp {
        object
            .key("Timestamp")
            .instant(var_16, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_crate_model_model(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Model,
) {
    if let Some(var_17) = &input.model_name {
        object.key("ModelName").string(var_17);
    }
    if let Some(var_18) = &input.model_version {
        object.key("ModelVersion").string(var_18);
    }
    if let Some(var_19) = &input.latest_sample_time {
        object
            .key("LatestSampleTime")
            .instant(var_19, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_20) = &input.latest_inference {
        object
            .key("LatestInference")
            .instant(var_20, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_21) = &input.model_metrics {
        let mut array_22 = object.key("ModelMetrics").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_edge_metric(
                    &mut object_24,
                    item_23,
                );
                object_24.finish();
            }
        }
        array_22.finish();
    }
}