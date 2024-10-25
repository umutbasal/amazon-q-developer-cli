// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_customization_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_customization::GetCustomizationOutput,
    crate::operation::get_customization::GetCustomizationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder =
        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
            .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_customization::GetCustomizationError::unhandled(
                generic,
            ));
        },
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::operation::get_customization::GetCustomizationError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::get_customization::GetCustomizationError::ThrottlingError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::get_customization::GetCustomizationError::ValidationError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::get_customization::GetCustomizationError::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundErrorBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
                };
                tmp
            })
        },
        "AccessDeniedException" => crate::operation::get_customization::GetCustomizationError::AccessDeniedError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_customization::GetCustomizationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_customization_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_customization::GetCustomizationOutput,
    crate::operation::get_customization::GetCustomizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_customization::builders::GetCustomizationOutputBuilder::default();
        output = crate::protocol_serde::shape_get_customization::de_get_customization(_response_body, output)
            .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_customization_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_customization::GetCustomizationError::unhandled)?
    })
}

pub fn ser_get_customization_input(
    input: &crate::operation::get_customization::GetCustomizationInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_customization_input::ser_get_customization_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_customization(
    value: &[u8],
    mut builder: crate::operation::get_customization::builders::GetCustomizationOutputBuilder,
) -> Result<
    crate::operation::get_customization::builders::GetCustomizationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "arn" => {
                    builder = builder.set_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "version" => {
                    builder = builder.set_version(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                },
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::CustomizationStatus::from(u.as_ref()))
                            })
                            .transpose()?,
                    );
                },
                "errorDetails" => {
                    builder = builder.set_error_details(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "dataReference" => {
                    builder = builder
                        .set_data_reference(crate::protocol_serde::shape_data_reference::de_data_reference(tokens)?);
                },
                "customizationName" => {
                    builder = builder.set_customization_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "description" => {
                    builder = builder.set_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "profileArn" => {
                    builder = builder.set_profile_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "updatedAt" => {
                    builder = builder.set_updated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                },
                "evaluationMetrics" => {
                    builder = builder.set_evaluation_metrics(
                        crate::protocol_serde::shape_evaluation_metrics::de_evaluation_metrics(tokens)?,
                    );
                },
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    format!("expected object key or end object, found: {:?}", other),
                ));
            },
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}