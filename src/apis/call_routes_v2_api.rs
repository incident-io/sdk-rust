/*
 * incident.io
 *
 * This is the API reference for incident.io.  It documents available API endpoints, provides examples of how to use it, and instructions around things like authentication and error handling.  The API is hosted at:  - https://api.incident.io/  And you will need to create an API key via your [incident.io dashboard](https://app.incident.io/settings/api-keys) to make requests.  # Making requests  Here are the key concepts required to make requests to the incident.io API.  ## Authentication  For all requests made to the incident.io API, you'll need an API key.  To create an API key, head to the incident dashboard and visit [API keys](https://app.incident.io/settings/api-keys). When you create the key, you'll be able to choose what actions it can take for your account: choose carefully, as those roles can only be set when you first create the key. We'll only show you the token once, so make sure you store it somewhere safe.  API keys are global to your incident.io account, and can be managed by anyone who has the right permissions. We display the user that created the API key, and the API key will remain valid if that user becomes deactivated.  Once you have the key, you should make requests to the API that set the `Authorization` request header using a \"Bearer\" authentication scheme:  ``` Authorization: Bearer <YOUR_API_KEY> ```  ## Rate Limits  The incident.io API enforces rate limits to ensure consistent performance for all users.  The default rate limit is 1200 requests/minute per API key. This limit applies to most endpoints across the API.  Limits are token buckets that refill continuously rather than resetting on a fixed window boundary. The default bucket holds 1200 requests and refills at 20 per second, so you can burst up to the full bucket and then sustain 20 requests/second indefinitely. There is no boundary at which your quota resets to full in one step.  Some endpoints have lower rate limits, particularly those that interact with external third-party systems that impose their own limitations. These specific limits vary by endpoint.  ### Rate limit headers  Responses to requests authenticated with an API key carry your current allowance, so you can pace yourself rather than waiting to be throttled:  ``` X-RateLimit-Limit: 60, 1200;window=60, 60;window=60 X-RateLimit-Remaining: 59 X-RateLimit-Used: 1 X-RateLimit-Reset: 1785173199 ```  | Header | Meaning | | --- | --- | | `X-RateLimit-Limit` | The quota that binds this request, followed by every limit that applied and the window it applies over | | `X-RateLimit-Remaining` | Requests you can make right now against the binding limit | | `X-RateLimit-Used` | Requests you have spent against it | | `X-RateLimit-Reset` | Unix timestamp (seconds) at which that limit will be back to full |  More than one limit can apply to a request: your API key's overall limit, and for some endpoints a lower limit of their own. `X-RateLimit-Limit` lists all of them, each with its window, so `1200;window=60` means 1200 requests per minute. Because our limits refill continuously rather than resetting on a boundary, that window is what tells you the rate you can sustain: 1200 per 60 seconds is 20 requests/second indefinitely.  `Remaining`, `Used` and `Reset` describe whichever limit has the least allowance left, since that is the one you will hit first.  `X-RateLimit-Remaining` may lag by a small number of requests under high concurrency, and can move by more than the requests you made, because limits scoped to your whole organisation are shared with your other API keys.  Headers are omitted rather than guessed if we cannot determine your allowance for a request.  ### Exceeding a rate limit  When you exceed a rate limit the API responds with `429 Too Many Requests` and a `Retry-After` header giving the number of seconds to wait:  ``` X-RateLimit-Limit: 1200, 1200;window=60 X-RateLimit-Remaining: 0 X-RateLimit-Used: 1200 X-RateLimit-Reset: 1785173199 Retry-After: 1 ```  Prefer `Retry-After` over `X-RateLimit-Reset` when deciding how long to back off. `Retry-After` is when a single request will succeed; `X-RateLimit-Reset` is the later point at which your whole allowance has returned. It is a duration rather than a timestamp, so it does not depend on your clock agreeing with ours.  The 429 also carries a JSON body with the same information:  ```json {     \"type\": \"too_many_requests\",     \"status\": 429,     \"request_id\": \"b839a403-7704-41c1-bf6a-39a2d68caefa\",     \"rate_limit\": {         \"name\": \"api_key_name\",         \"limit\": 1200,         \"remaining\": 0,         \"retry_after\": \"2025-04-17T11:17:18Z\"     },     \"errors\": [         {             \"code\": \"too_many_requests\",             \"message\": \"Too many requests hit the API too quickly. We recommend an exponential backoff of your requests.\"         }     ] } ```  The response includes: * The name of the API key (`name`) * The bucket limit (`limit`) * The number of requests remaining (`remaining`) * When you can retry requests (`retry_after`), as an RFC3339 timestamp  ## Errors  We use standard HTTP response codes to indicate the status or failure of API requests.  The API response body will be JSON, and contain more detailed information on the nature of the error.  An example error when a request is made without an API key:  ```json {   \"type\": \"authentication_error\",   \"status\": 401,   \"request_id\": \"8e3cc412-b49d-4957-9073-2c19d2c61804\",   \"errors\": [     {       \"code\": \"missing_authorization_material\",       \"message\": \"No authorization material provided in request\"     }   ] } ```  Note that the error:  - Contains the HTTP status (`401`) - References the type of error (`authentication_error`) - Includes a `request_id` that can be provided to incident.io support to help  debug questions with your API request - Provides a list of individual errors, which go into detail about why the error  occurred  The most common error will be a 422 Validation Error, which is returned when the request was rejected due to failing validations.  These errors look like this:  ```json {   \"type\": \"validation_error\",   \"status\": 422,   \"request_id\": \"631766c4-4afd-4803-997c-cd700928fa4b\",   \"errors\": [     {       \"code\": \"is_required\",       \"message\": \"A severity is required to open an incident\",       \"source\": {         \"field\": \"severity_id\"       }     }   ] } ```  This error is caused by not providing a severity identifier, which should be at the `severity_id` field of the request payload. Errors like these can be mapped to forms, should you be integrating with the API from a user-interface.  ## Compatibility  We won't make breaking changes to existing API services or endpoints, but will expect integrators to upgrade themselves to the latest API endpoints within 3 months of us deprecating the old service.  We will make changes that are considered backwards compatible, which include:  - Adding new API endpoints and services - Adding new properties to responses from existing API endpoints - Reordering properties returned from existing API endpoints - Adding optional request parameters to existing API endpoints - Altering the format or length of IDs - Adding new values to enums  It is important that clients are robust to these changes, to ensure reliable integrations.  As an example, if you are generating a client using an openapi-generator, ensure the generated client is configured to support unknown enum values, often configured via the `enumUnknownDefaultCase` parameter.  When breaking changes are unavoidable, we'll create a new service version on a separate path, and run them in parallel.  For example:  - https://api.incident.io/v1/incidents - https://api.incident.io/v2/incidents  For any questions, email support@incident.io.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`call_routes_v2_create_allowed_caller`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2CreateAllowedCallerParams {
    /// The call route to allow this number to call
    pub call_route_id: String,
    pub call_routes_create_allowed_caller_payload_v2:
        models::CallRoutesCreateAllowedCallerPayloadV2,
}

/// struct for passing parameters to the method [`call_routes_v2_create_option`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2CreateOptionParams {
    /// The call route to add this option to
    pub call_route_id: String,
    pub call_routes_create_option_payload_v2: models::CallRoutesCreateOptionPayloadV2,
}

/// struct for passing parameters to the method [`call_routes_v2_destroy_allowed_caller`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2DestroyAllowedCallerParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The allowed caller's ID
    pub id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_destroy_option`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2DestroyOptionParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The option's ID
    pub id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ListParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// A call route's ID. This endpoint will return a list of call routes after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`call_routes_v2_list_allowed_callers`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ListAllowedCallersParams {
    /// The call route's ID
    pub call_route_id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_list_options`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ListOptionsParams {
    /// The call route's ID
    pub call_route_id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ShowParams {
    /// The call route's ID
    pub id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_show_allowed_caller`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ShowAllowedCallerParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The allowed caller's ID
    pub id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_show_option`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2ShowOptionParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The option's ID
    pub id: String,
}

/// struct for passing parameters to the method [`call_routes_v2_update`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2UpdateParams {
    /// The call route's ID
    pub id: String,
    pub call_routes_update_payload_v2: models::CallRoutesUpdatePayloadV2,
}

/// struct for passing parameters to the method [`call_routes_v2_update_allowed_caller`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2UpdateAllowedCallerParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The allowed caller's ID
    pub id: String,
    pub call_routes_update_allowed_caller_payload_v2:
        models::CallRoutesUpdateAllowedCallerPayloadV2,
}

/// struct for passing parameters to the method [`call_routes_v2_update_option`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CallRoutesV2UpdateOptionParams {
    /// The call route's ID
    pub call_route_id: String,
    /// The option's ID
    pub id: String,
    pub call_routes_update_option_payload_v2: models::CallRoutesUpdateOptionPayloadV2,
}

/// struct for typed errors of method [`call_routes_v2_create_allowed_caller`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2CreateAllowedCallerError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_create_option`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2CreateOptionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_destroy_allowed_caller`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2DestroyAllowedCallerError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_destroy_option`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2DestroyOptionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ListError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_list_allowed_callers`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ListAllowedCallersError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_list_options`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ListOptionsError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ShowError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_show_allowed_caller`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ShowAllowedCallerError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_show_option`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2ShowOptionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_update`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2UpdateError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_update_allowed_caller`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2UpdateAllowedCallerError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`call_routes_v2_update_option`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallRoutesV2UpdateOptionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status408(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status412(models::ErrorResponse),
    Status413(models::ErrorResponse),
    Status422(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Allow a number to call this route.
pub async fn call_routes_v2_create_allowed_caller(
    configuration: &configuration::Configuration,
    params: CallRoutesV2CreateAllowedCallerParams,
) -> Result<
    models::CallRoutesCreateAllowedCallerResultV2,
    Error<CallRoutesV2CreateAllowedCallerError>,
> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/allowed_callers",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.call_routes_create_allowed_caller_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesCreateAllowedCallerResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesCreateAllowedCallerResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2CreateAllowedCallerError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| CallRoutesV2CreateAllowedCallerError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Add an option to a call route's phone-tree menu.  Callers hear the menu instead of being routed down the route's own path, so adding the first option clears that path. Every plan allows a single option. A menu of two or more options is not on every plan, so get in touch if you need one enabled.
pub async fn call_routes_v2_create_option(
    configuration: &configuration::Configuration,
    params: CallRoutesV2CreateOptionParams,
) -> Result<models::CallRoutesCreateOptionResultV2, Error<CallRoutesV2CreateOptionError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/options",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.call_routes_create_option_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesCreateOptionResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesCreateOptionResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2CreateOptionError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2CreateOptionError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Stop allowing a number to call this route.  A route that only answers its allowed callers must keep at least one, so turn use_caller_allowlist off before removing the last number.
pub async fn call_routes_v2_destroy_allowed_caller(
    configuration: &configuration::Configuration,
    params: CallRoutesV2DestroyAllowedCallerParams,
) -> Result<(), Error<CallRoutesV2DestroyAllowedCallerError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/allowed_callers/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2DestroyAllowedCallerError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| CallRoutesV2DestroyAllowedCallerError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Remove an option from a call route's phone-tree menu.  A route has to route calls somewhere, so give it a path before removing its last option.
pub async fn call_routes_v2_destroy_option(
    configuration: &configuration::Configuration,
    params: CallRoutesV2DestroyOptionParams,
) -> Result<(), Error<CallRoutesV2DestroyOptionError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/options/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2DestroyOptionError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2DestroyOptionError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all call routes for this organisation.
pub async fn call_routes_v2_list(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ListParams,
) -> Result<models::CallRoutesListResultV2, Error<CallRoutesV2ListError>> {
    let uri_str = format!("{}/v2/call_routes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the numbers allowed to call a route.
pub async fn call_routes_v2_list_allowed_callers(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ListAllowedCallersParams,
) -> Result<models::CallRoutesListAllowedCallersResultV2, Error<CallRoutesV2ListAllowedCallersError>>
{
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/allowed_callers",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesListAllowedCallersResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesListAllowedCallersResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ListAllowedCallersError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| CallRoutesV2ListAllowedCallersError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List a call route's phone-tree options, in the order callers hear them.
pub async fn call_routes_v2_list_options(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ListOptionsParams,
) -> Result<models::CallRoutesListOptionsResultV2, Error<CallRoutesV2ListOptionsError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/options",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesListOptionsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesListOptionsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ListOptionsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2ListOptionsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single call route.
pub async fn call_routes_v2_show(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ShowParams,
) -> Result<models::CallRoutesShowResultV2, Error<CallRoutesV2ShowError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single allowed caller.
pub async fn call_routes_v2_show_allowed_caller(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ShowAllowedCallerParams,
) -> Result<models::CallRoutesShowAllowedCallerResultV2, Error<CallRoutesV2ShowAllowedCallerError>>
{
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/allowed_callers/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesShowAllowedCallerResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesShowAllowedCallerResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ShowAllowedCallerError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2ShowAllowedCallerError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single phone-tree option.
pub async fn call_routes_v2_show_option(
    configuration: &configuration::Configuration,
    params: CallRoutesV2ShowOptionParams,
) -> Result<models::CallRoutesShowOptionResultV2, Error<CallRoutesV2ShowOptionError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/options/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesShowOptionResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesShowOptionResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2ShowOptionError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2ShowOptionError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Replace a call route's configuration.  Sending a path retires any phone-tree menu on the route. Send an empty path to keep the menu.
pub async fn call_routes_v2_update(
    configuration: &configuration::Configuration,
    params: CallRoutesV2UpdateParams,
) -> Result<models::CallRoutesUpdateResultV2, Error<CallRoutesV2UpdateError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.call_routes_update_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesUpdateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesUpdateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2UpdateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2UpdateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Replace an allowed caller's number and label.
pub async fn call_routes_v2_update_allowed_caller(
    configuration: &configuration::Configuration,
    params: CallRoutesV2UpdateAllowedCallerParams,
) -> Result<
    models::CallRoutesUpdateAllowedCallerResultV2,
    Error<CallRoutesV2UpdateAllowedCallerError>,
> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/allowed_callers/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.call_routes_update_allowed_caller_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesUpdateAllowedCallerResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesUpdateAllowedCallerResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2UpdateAllowedCallerError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| CallRoutesV2UpdateAllowedCallerError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Replace a phone-tree option.  Two live options can't share a digit, so moving an option onto a digit the menu already uses is rejected.
pub async fn call_routes_v2_update_option(
    configuration: &configuration::Configuration,
    params: CallRoutesV2UpdateOptionParams,
) -> Result<models::CallRoutesUpdateOptionResultV2, Error<CallRoutesV2UpdateOptionError>> {
    let uri_str = format!(
        "{}/v2/call_routes/{call_route_id}/options/{id}",
        configuration.base_path,
        call_route_id = crate::apis::urlencode(params.call_route_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.call_routes_update_option_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content)).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CallRoutesUpdateOptionResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CallRoutesUpdateOptionResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CallRoutesV2UpdateOptionError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| CallRoutesV2UpdateOptionError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl CallRoutesV2CreateAllowedCallerParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        call_route_id: impl Into<String>,
        call_routes_create_allowed_caller_payload_v2: models::CallRoutesCreateAllowedCallerPayloadV2,
    ) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            call_routes_create_allowed_caller_payload_v2,
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `call_routes_create_allowed_caller_payload_v2`.
    #[must_use]
    pub fn set_call_routes_create_allowed_caller_payload_v2(
        mut self,
        value: models::CallRoutesCreateAllowedCallerPayloadV2,
    ) -> Self {
        self.call_routes_create_allowed_caller_payload_v2 = value;
        self
    }
}

impl CallRoutesV2CreateOptionParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        call_route_id: impl Into<String>,
        call_routes_create_option_payload_v2: models::CallRoutesCreateOptionPayloadV2,
    ) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            call_routes_create_option_payload_v2,
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `call_routes_create_option_payload_v2`.
    #[must_use]
    pub fn set_call_routes_create_option_payload_v2(
        mut self,
        value: models::CallRoutesCreateOptionPayloadV2,
    ) -> Self {
        self.call_routes_create_option_payload_v2 = value;
        self
    }
}

impl CallRoutesV2DestroyAllowedCallerParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl CallRoutesV2DestroyOptionParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl CallRoutesV2ListParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {
            page_size: None,
            after: None,
        }
    }

    /// Sets `page_size`.
    #[must_use]
    pub fn set_page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Sets `after`.
    #[must_use]
    pub fn set_after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }
}

impl CallRoutesV2ListAllowedCallersParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }
}

impl CallRoutesV2ListOptionsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }
}

impl CallRoutesV2ShowParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl CallRoutesV2ShowAllowedCallerParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl CallRoutesV2ShowOptionParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(call_route_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl CallRoutesV2UpdateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        call_routes_update_payload_v2: models::CallRoutesUpdatePayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            call_routes_update_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `call_routes_update_payload_v2`.
    #[must_use]
    pub fn set_call_routes_update_payload_v2(
        mut self,
        value: models::CallRoutesUpdatePayloadV2,
    ) -> Self {
        self.call_routes_update_payload_v2 = value;
        self
    }
}

impl CallRoutesV2UpdateAllowedCallerParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        call_route_id: impl Into<String>,
        id: impl Into<String>,
        call_routes_update_allowed_caller_payload_v2: models::CallRoutesUpdateAllowedCallerPayloadV2,
    ) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
            call_routes_update_allowed_caller_payload_v2,
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `call_routes_update_allowed_caller_payload_v2`.
    #[must_use]
    pub fn set_call_routes_update_allowed_caller_payload_v2(
        mut self,
        value: models::CallRoutesUpdateAllowedCallerPayloadV2,
    ) -> Self {
        self.call_routes_update_allowed_caller_payload_v2 = value;
        self
    }
}

impl CallRoutesV2UpdateOptionParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        call_route_id: impl Into<String>,
        id: impl Into<String>,
        call_routes_update_option_payload_v2: models::CallRoutesUpdateOptionPayloadV2,
    ) -> Self {
        Self {
            call_route_id: call_route_id.into(),
            id: id.into(),
            call_routes_update_option_payload_v2,
        }
    }

    /// Sets `call_route_id`.
    #[must_use]
    pub fn set_call_route_id(mut self, value: impl Into<String>) -> Self {
        self.call_route_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `call_routes_update_option_payload_v2`.
    #[must_use]
    pub fn set_call_routes_update_option_payload_v2(
        mut self,
        value: models::CallRoutesUpdateOptionPayloadV2,
    ) -> Self {
        self.call_routes_update_option_payload_v2 = value;
        self
    }
}

impl Default for CallRoutesV2ListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl CallRoutesV2CreateAllowedCallerError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2CreateOptionError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2DestroyAllowedCallerError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2DestroyOptionError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ListError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ListAllowedCallersError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ListOptionsError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ShowError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ShowAllowedCallerError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2ShowOptionError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2UpdateError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2UpdateAllowedCallerError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}

impl CallRoutesV2UpdateOptionError {
    /// The variant matching the response's HTTP status.
    ///
    /// Not `serde`: every variant holds the same type and the enum
    /// is `#[serde(untagged)]`, so deserializing would always return
    /// the lowest status code the endpoint documents.
    fn from_status(status: u16, body: models::ErrorResponse) -> Self {
        match status {
            400 => Self::Status400(body),
            401 => Self::Status401(body),
            403 => Self::Status403(body),
            404 => Self::Status404(body),
            405 => Self::Status405(body),
            406 => Self::Status406(body),
            408 => Self::Status408(body),
            409 => Self::Status409(body),
            412 => Self::Status412(body),
            413 => Self::Status413(body),
            422 => Self::Status422(body),
            429 => Self::Status429(body),
            500 => Self::Status500(body),
            _ => Self::UnknownValue(serde_json::to_value(body).unwrap_or(serde_json::Value::Null)),
        }
    }
}
