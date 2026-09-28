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

/// struct for passing parameters to the method [`escalations_v2_cancel_escalation`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2CancelEscalationParams {
    /// Unique ID of the escalation
    pub id: String,
}

/// struct for passing parameters to the method [`escalations_v2_check_escalation_permissions`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2CheckEscalationPermissionsParams {
    /// The ID of the escalation
    pub escalation_id: String,
    pub escalations_check_escalation_permissions_payload_v2:
        models::EscalationsCheckEscalationPermissionsPayloadV2,
}

/// struct for passing parameters to the method [`escalations_v2_create`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2CreateParams {
    pub escalations_create_payload_v2: models::EscalationsCreatePayloadV2,
}

/// struct for passing parameters to the method [`escalations_v2_create_path`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2CreatePathParams {
    pub escalations_create_path_payload_v2: models::EscalationsCreatePathPayloadV2,
}

/// struct for passing parameters to the method [`escalations_v2_destroy_path`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2DestroyPathParams {
    /// Unique identifier for this escalation path.
    pub id: String,
}

/// struct for passing parameters to the method [`escalations_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2ListParams {
    /// Number of escalations to return per page
    pub page_size: Option<i64>,
    /// An escalation's ID. This endpoint will return a list of escalations after this ID in relation to the API response order.
    pub after: Option<String>,
    /// Filter on the escalation path for which the escalation was triggered. Accepted operators are 'one_of' and 'not_in'.
    pub escalation_path: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the status of the escalation. Accepted operators are 'one_of' and 'not_in'.
    pub status: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the alert that created an escalation. Accepted operators are 'one_of' and 'not_in'.
    pub alert: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the incident that the escalation is connected to. Accepted operators are 'one_of' and 'not_in'.
    pub incident: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the created_at timestamp of the escalation. Accepted operators are 'gte', 'lte' and 'date_range'.
    pub created_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the updated_at timestamp of the escalation. Accepted operators are 'gte', 'lte' and 'date_range'.
    pub updated_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the idempotency key of the escalation. This is the key set when creating escalations via the API, and is distinct from alert deduplication keys. Accepted operators are 'is' for exact matches and 'starts_with' for prefix matching.
    pub idempotency_key: Option<std::collections::HashMap<String, Vec<String>>>,
}

/// struct for passing parameters to the method [`escalations_v2_list_paths`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2ListPathsParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An record's ID. This endpoint will return a list of records after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`escalations_v2_reassign_escalation`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2ReassignEscalationParams {
    /// The ID of the escalation to reassign
    pub escalation_id: String,
    pub escalations_reassign_escalation_payload_v2: models::EscalationsReassignEscalationPayloadV2,
}

/// struct for passing parameters to the method [`escalations_v2_respond_escalation`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2RespondEscalationParams {
    /// The ID of the escalation
    pub escalation_id: String,
    pub escalations_respond_escalation_payload_v2: models::EscalationsRespondEscalationPayloadV2,
}

/// struct for passing parameters to the method [`escalations_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2ShowParams {
    /// Unique ID of the escalation
    pub id: String,
}

/// struct for passing parameters to the method [`escalations_v2_show_path`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2ShowPathParams {
    /// Unique identifier for this escalation path.
    pub id: String,
}

/// struct for passing parameters to the method [`escalations_v2_update_path`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EscalationsV2UpdatePathParams {
    /// Unique identifier for this escalation path.
    pub id: String,
    pub escalations_update_path_payload_v2: models::EscalationsUpdatePathPayloadV2,
}

/// struct for typed errors of method [`escalations_v2_cancel_escalation`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2CancelEscalationError {
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

/// struct for typed errors of method [`escalations_v2_check_escalation_permissions`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2CheckEscalationPermissionsError {
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

/// struct for typed errors of method [`escalations_v2_create`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2CreateError {
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

/// struct for typed errors of method [`escalations_v2_create_path`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2CreatePathError {
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

/// struct for typed errors of method [`escalations_v2_destroy_path`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2DestroyPathError {
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

/// struct for typed errors of method [`escalations_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2ListError {
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

/// struct for typed errors of method [`escalations_v2_list_paths`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2ListPathsError {
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

/// struct for typed errors of method [`escalations_v2_reassign_escalation`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2ReassignEscalationError {
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

/// struct for typed errors of method [`escalations_v2_respond_escalation`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2RespondEscalationError {
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

/// struct for typed errors of method [`escalations_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2ShowError {
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

/// struct for typed errors of method [`escalations_v2_show_path`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2ShowPathError {
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

/// struct for typed errors of method [`escalations_v2_update_path`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscalationsV2UpdatePathError {
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

/// Cancel an escalation.  Cancelling an escalation stops any further paging: notifications cease and the escalation will not advance to further levels or repeat. This works on escalations that are still paging (for example to silence a page when an incident is resolved before anyone acknowledges it) as well as snoozed ones. Escalations that have already resolved or expired cannot be cancelled, and cancelling an already-cancelled escalation is a no-op.  To use this API, you will need an API key with the \"Create and manage escalations\" permission.
pub async fn escalations_v2_cancel_escalation(
    configuration: &configuration::Configuration,
    params: EscalationsV2CancelEscalationParams,
) -> Result<(), Error<EscalationsV2CancelEscalationError>> {
    let uri_str = format!(
        "{}/v2/escalations/{id}/actions/cancel",
        configuration.base_path,
        id = crate::apis::urlencode(params.id)
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

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2CancelEscalationError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2CancelEscalationError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Check whether the given users can currently acknowledge, decline, or snooze an escalation.  This is a read-only projection of the escalation's current state, intended for deciding which response actions to surface to each user in a custom integration.  To use this API, you will need an API key that can view escalations and users.
pub async fn escalations_v2_check_escalation_permissions(
    configuration: &configuration::Configuration,
    params: EscalationsV2CheckEscalationPermissionsParams,
) -> Result<
    models::EscalationsCheckEscalationPermissionsResultV2,
    Error<EscalationsV2CheckEscalationPermissionsError>,
> {
    let uri_str = format!(
        "{}/v2/escalations/{escalation_id}/actions/check_permissions",
        configuration.base_path,
        escalation_id = crate::apis::urlencode(params.escalation_id)
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
    req_builder = req_builder.json(&params.escalations_check_escalation_permissions_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsCheckEscalationPermissionsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsCheckEscalationPermissionsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2CheckEscalationPermissionsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| {
                    EscalationsV2CheckEscalationPermissionsError::from_status(status.as_u16(), body)
                });
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create an escalation.  An escalation pages people, either according to an escalation path, or directly to specific users. You must provide either an escalation_path_id OR user_ids, but not both.  When escalating via an escalation path, the escalation will follow the configured path with its levels and timeouts, using your default [alert priority](https://app.incident.io/~/settings/alerts/configuration/priorities).  When escalating directly to users, they will receive a high-urgency notification, based on their notification rules.  This endpoint is rate-limited to 60 requests per minute, since it is intended for interactive use cases (for example someone clicking a \"escalate to team\" button in your internal developer platform). To escalate based on automated alerts, we recommend sending events to an alert source instead.  If your API key's permissions are scoped to teams, you can only escalate via an escalation path that one of those teams owns. Escalating directly to user_ids needs the permission at the account level, because an escalation aimed at a person has no owning team.
pub async fn escalations_v2_create(
    configuration: &configuration::Configuration,
    params: EscalationsV2CreateParams,
) -> Result<models::EscalationsCreateResultV2, Error<EscalationsV2CreateError>> {
    let uri_str = format!("{}/v2/escalations", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.escalations_create_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsCreateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsCreateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2CreateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2CreateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create an escalation path.  An escalation path is a series of steps that describe how a page should be escalated, represented as graph, supporting conditional branches based on alert priority and working intervals.  We recommend you create escalation paths in the incident.io dashboard where our path builder makes it easy to use conditions and visualise the path.
pub async fn escalations_v2_create_path(
    configuration: &configuration::Configuration,
    params: EscalationsV2CreatePathParams,
) -> Result<models::EscalationsCreatePathResultV2, Error<EscalationsV2CreatePathError>> {
    let uri_str = format!("{}/v2/escalation_paths", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.escalations_create_path_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsCreatePathResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsCreatePathResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2CreatePathError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2CreatePathError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archives an escalation path.  We recommend you create escalation paths in the incident.io dashboard where our path builder makes it easy to use conditions and visualise the path.
pub async fn escalations_v2_destroy_path(
    configuration: &configuration::Configuration,
    params: EscalationsV2DestroyPathParams,
) -> Result<(), Error<EscalationsV2DestroyPathError>> {
    let uri_str = format!(
        "{}/v2/escalation_paths/{id}",
        configuration.base_path,
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
        let entity: Option<EscalationsV2DestroyPathError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2DestroyPathError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all escalations for your account.  This endpoint supports a number of filters, which can help find escalations matching certain criteria.  Note that: - Filters may be used together, and the result will be escalations that match all filters. - All query parameters must be URI encoded.  To use this API, you will need an API key with the \"View data\" or \"Create and manage on-call resources\" permission.  ### By escalation_path  Find all escalations that escalated to escalation path with id=ABC:    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'escalation_path[one_of]=ABC'  ### By status  Find all escalations with a current status of \"triggered\":    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'status[one_of]=triggered'  Possible values are \"pending\", \"triggered\", \"acked\", \"resolved\", \"expired\" and \"cancelled\". Escalations are in \"pending\" when they are in a grace period when the related alert has been grouped in an incident.  ### By alert  Find all escalations that were created by alert with id=ABC:    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'alert[one_of]=ABC'  ### By incident  Find all escalations related to incident with id=ABC:    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'incident[one_of]=ABC'  An escalation is related to an incident if it is linked to that incident directly, if it is attached to one of the incident's alerts, or if it triggered one of those alerts (which is what happens when someone pages by calling in, and that call raises the alert).  To find everything that is not related to an incident, use \"not_in\":    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'incident[not_in]=ABC'  ### By created_at and updated_at Find all escalations that follow specified date parameters for created_at and updated_at fields. Possible values are \"gte\" (greater than or equal to), \"lte\" (less than or equal to), and \"date_range\" (between two dates). For example, to find all escalations updated after 2025-01-01:    curl --get 'https://api.incident.io/v2/escalations' \\    --data 'updated_at[gte]=2025-01-01'  To find all escalations created between 2025-01-01 and 2025-01-31:    curl --get 'https://api.incident.io/v2/escalations' \\             --data 'created_at[date_range]=2025-01-01~2025-01-31'
pub async fn escalations_v2_list(
    configuration: &configuration::Configuration,
    params: EscalationsV2ListParams,
) -> Result<models::EscalationsListResultV2, Error<EscalationsV2ListError>> {
    let uri_str = format!("{}/v2/escalations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.escalation_path {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "escalation_path",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.status {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "status",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.alert {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "alert",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.incident {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "incident",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.created_at {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "created_at",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.updated_at {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "updated_at",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.idempotency_key {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "idempotency_key",
            &serde_json::to_value(param_value)?,
        ));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all escalation paths in your account.  An escalation path is a series of steps that describe how a page should be escalated, represented as a graph, supporting conditional branches based on alert priority and working intervals.
pub async fn escalations_v2_list_paths(
    configuration: &configuration::Configuration,
    params: EscalationsV2ListPathsParams,
) -> Result<models::EscalationsListPathsResultV2, Error<EscalationsV2ListPathsError>> {
    let uri_str = format!("{}/v2/escalation_paths", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsListPathsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsListPathsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2ListPathsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2ListPathsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Reassign an escalation to a different escalation path or set of users.  Use this when a page reached the wrong people. Reassigning creates a new escalation aimed at the targets you give, linked back to the original, and carries over the original's title, description, priority, incident and alert so the page stays attached to the same alert and acknowledgements keep grouping under it.  You must provide either an escalation_path_id OR user_ids, but not both. Both name incident.io escalation paths and users, so an escalation cannot be reassigned to an external escalator such as PagerDuty or Opsgenie.  By default the original escalation is resolved, so the first targets stop being paged. Pass resolve_original=false to leave it running alongside the new one. Resolving the original additionally needs the \"escalations.respond\" permission, since it stops a page someone else may be responding to.  An escalation can only be reassigned once: calling this again for the same escalation is rejected. Retrying a request whose response you never saw is safe in the sense that matters — a reassignment that already happened is never repeated, so nobody is paged twice — but the retry is rejected rather than returning the escalation the first call created, so keep the ID from the original response.  To use this API, you will need an API key with the \"Create and manage escalations\" permission. The reassignment is attributed to the key; to attribute it to one of your users instead, set the X-Incident-User header, which needs the \"api_keys.act_on_behalf_of_users\" scope.  If your API key's permissions are scoped to teams, the escalation you're reassigning must be owned by one of those teams, and you can only reassign via an escalation path one of them owns. Reassigning directly to user_ids needs the permission at the account level, because an escalation aimed at a person has no owning team.
pub async fn escalations_v2_reassign_escalation(
    configuration: &configuration::Configuration,
    params: EscalationsV2ReassignEscalationParams,
) -> Result<
    models::EscalationsReassignEscalationResultV2,
    Error<EscalationsV2ReassignEscalationError>,
> {
    let uri_str = format!(
        "{}/v2/escalations/{escalation_id}/actions/reassign",
        configuration.base_path,
        escalation_id = crate::apis::urlencode(params.escalation_id)
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
    req_builder = req_builder.json(&params.escalations_reassign_escalation_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsReassignEscalationResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsReassignEscalationResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2ReassignEscalationError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| EscalationsV2ReassignEscalationError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Respond to an escalation.  An API key can acknowledge or snooze an escalation on its own, and the response is attributed to the key.  Declining (\"nack\") means \"I personally can't take this page\", so it isn't available to an API key responding in its own right.  To use this API, you will need an API key with the \"Create and manage escalations\" permission.
pub async fn escalations_v2_respond_escalation(
    configuration: &configuration::Configuration,
    params: EscalationsV2RespondEscalationParams,
) -> Result<(), Error<EscalationsV2RespondEscalationError>> {
    let uri_str = format!(
        "{}/v2/escalations/{escalation_id}/actions/respond",
        configuration.base_path,
        escalation_id = crate::apis::urlencode(params.escalation_id)
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
    req_builder = req_builder.json(&params.escalations_respond_escalation_payload_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2RespondEscalationError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| EscalationsV2RespondEscalationError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a specific escalation.
pub async fn escalations_v2_show(
    configuration: &configuration::Configuration,
    params: EscalationsV2ShowParams,
) -> Result<models::EscalationsShowResultV2, Error<EscalationsV2ShowError>> {
    let uri_str = format!(
        "{}/v2/escalations/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show an escalation path.  We recommend you create escalation paths in the incident.io dashboard where our path builder makes it easy to use conditions and visualise the path.
pub async fn escalations_v2_show_path(
    configuration: &configuration::Configuration,
    params: EscalationsV2ShowPathParams,
) -> Result<models::EscalationsShowPathResultV2, Error<EscalationsV2ShowPathError>> {
    let uri_str = format!(
        "{}/v2/escalation_paths/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsShowPathResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsShowPathResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2ShowPathError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2ShowPathError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Updates an escalation path.  We recommend you create escalation paths in the incident.io dashboard where our path builder makes it easy to use conditions and visualise the path.
pub async fn escalations_v2_update_path(
    configuration: &configuration::Configuration,
    params: EscalationsV2UpdatePathParams,
) -> Result<models::EscalationsUpdatePathResultV2, Error<EscalationsV2UpdatePathError>> {
    let uri_str = format!(
        "{}/v2/escalation_paths/{id}",
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
    req_builder = req_builder.json(&params.escalations_update_path_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EscalationsUpdatePathResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EscalationsUpdatePathResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EscalationsV2UpdatePathError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| EscalationsV2UpdatePathError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl EscalationsV2CancelEscalationParams {
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

impl EscalationsV2CheckEscalationPermissionsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        escalation_id: impl Into<String>,
        escalations_check_escalation_permissions_payload_v2: models::EscalationsCheckEscalationPermissionsPayloadV2,
    ) -> Self {
        Self {
            escalation_id: escalation_id.into(),
            escalations_check_escalation_permissions_payload_v2,
        }
    }

    /// Sets `escalation_id`.
    #[must_use]
    pub fn set_escalation_id(mut self, value: impl Into<String>) -> Self {
        self.escalation_id = value.into();
        self
    }

    /// Sets `escalations_check_escalation_permissions_payload_v2`.
    #[must_use]
    pub fn set_escalations_check_escalation_permissions_payload_v2(
        mut self,
        value: models::EscalationsCheckEscalationPermissionsPayloadV2,
    ) -> Self {
        self.escalations_check_escalation_permissions_payload_v2 = value;
        self
    }
}

impl EscalationsV2CreateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(escalations_create_payload_v2: models::EscalationsCreatePayloadV2) -> Self {
        Self {
            escalations_create_payload_v2,
        }
    }

    /// Sets `escalations_create_payload_v2`.
    #[must_use]
    pub fn set_escalations_create_payload_v2(
        mut self,
        value: models::EscalationsCreatePayloadV2,
    ) -> Self {
        self.escalations_create_payload_v2 = value;
        self
    }
}

impl EscalationsV2CreatePathParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(escalations_create_path_payload_v2: models::EscalationsCreatePathPayloadV2) -> Self {
        Self {
            escalations_create_path_payload_v2,
        }
    }

    /// Sets `escalations_create_path_payload_v2`.
    #[must_use]
    pub fn set_escalations_create_path_payload_v2(
        mut self,
        value: models::EscalationsCreatePathPayloadV2,
    ) -> Self {
        self.escalations_create_path_payload_v2 = value;
        self
    }
}

impl EscalationsV2DestroyPathParams {
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

impl EscalationsV2ListParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {
            page_size: None,
            after: None,
            escalation_path: None,
            status: None,
            alert: None,
            incident: None,
            created_at: None,
            updated_at: None,
            idempotency_key: None,
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

    /// Sets `escalation_path`.
    #[must_use]
    pub fn set_escalation_path(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.escalation_path = Some(value);
        self
    }

    /// Sets `status`.
    #[must_use]
    pub fn set_status(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets `alert`.
    #[must_use]
    pub fn set_alert(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.alert = Some(value);
        self
    }

    /// Sets `incident`.
    #[must_use]
    pub fn set_incident(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.incident = Some(value);
        self
    }

    /// Sets `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Sets `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Sets `idempotency_key`.
    #[must_use]
    pub fn set_idempotency_key(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.idempotency_key = Some(value);
        self
    }
}

impl EscalationsV2ListPathsParams {
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

impl EscalationsV2ReassignEscalationParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        escalation_id: impl Into<String>,
        escalations_reassign_escalation_payload_v2: models::EscalationsReassignEscalationPayloadV2,
    ) -> Self {
        Self {
            escalation_id: escalation_id.into(),
            escalations_reassign_escalation_payload_v2,
        }
    }

    /// Sets `escalation_id`.
    #[must_use]
    pub fn set_escalation_id(mut self, value: impl Into<String>) -> Self {
        self.escalation_id = value.into();
        self
    }

    /// Sets `escalations_reassign_escalation_payload_v2`.
    #[must_use]
    pub fn set_escalations_reassign_escalation_payload_v2(
        mut self,
        value: models::EscalationsReassignEscalationPayloadV2,
    ) -> Self {
        self.escalations_reassign_escalation_payload_v2 = value;
        self
    }
}

impl EscalationsV2RespondEscalationParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        escalation_id: impl Into<String>,
        escalations_respond_escalation_payload_v2: models::EscalationsRespondEscalationPayloadV2,
    ) -> Self {
        Self {
            escalation_id: escalation_id.into(),
            escalations_respond_escalation_payload_v2,
        }
    }

    /// Sets `escalation_id`.
    #[must_use]
    pub fn set_escalation_id(mut self, value: impl Into<String>) -> Self {
        self.escalation_id = value.into();
        self
    }

    /// Sets `escalations_respond_escalation_payload_v2`.
    #[must_use]
    pub fn set_escalations_respond_escalation_payload_v2(
        mut self,
        value: models::EscalationsRespondEscalationPayloadV2,
    ) -> Self {
        self.escalations_respond_escalation_payload_v2 = value;
        self
    }
}

impl EscalationsV2ShowParams {
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

impl EscalationsV2ShowPathParams {
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

impl EscalationsV2UpdatePathParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        escalations_update_path_payload_v2: models::EscalationsUpdatePathPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            escalations_update_path_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `escalations_update_path_payload_v2`.
    #[must_use]
    pub fn set_escalations_update_path_payload_v2(
        mut self,
        value: models::EscalationsUpdatePathPayloadV2,
    ) -> Self {
        self.escalations_update_path_payload_v2 = value;
        self
    }
}

impl Default for EscalationsV2ListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EscalationsV2ListPathsParams {
    fn default() -> Self {
        Self::new()
    }
}

impl EscalationsV2CancelEscalationError {
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

impl EscalationsV2CheckEscalationPermissionsError {
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

impl EscalationsV2CreateError {
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

impl EscalationsV2CreatePathError {
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

impl EscalationsV2DestroyPathError {
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

impl EscalationsV2ListError {
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

impl EscalationsV2ListPathsError {
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

impl EscalationsV2ReassignEscalationError {
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

impl EscalationsV2RespondEscalationError {
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

impl EscalationsV2ShowError {
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

impl EscalationsV2ShowPathError {
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

impl EscalationsV2UpdatePathError {
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
