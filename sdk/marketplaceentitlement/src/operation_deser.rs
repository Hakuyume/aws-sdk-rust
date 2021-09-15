// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_entitlements_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetEntitlementsOutput, crate::error::GetEntitlementsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetEntitlementsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetEntitlementsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceErrorException" => crate::error::GetEntitlementsError {
            meta: generic,
            kind: crate::error::GetEntitlementsErrorKind::InternalServiceErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::internal_service_error_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_error_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetEntitlementsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidParameterException" => crate::error::GetEntitlementsError {
            meta: generic,
            kind: crate::error::GetEntitlementsErrorKind::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_parameter_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetEntitlementsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::GetEntitlementsError {
            meta: generic,
            kind: crate::error::GetEntitlementsErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetEntitlementsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetEntitlementsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_entitlements_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetEntitlementsOutput, crate::error::GetEntitlementsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_entitlements_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_entitlements(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetEntitlementsError::unhandled)?;
        output.build()
    })
}