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

/// struct for passing parameters to the method [`status_pages_v2_create_status_page_incident`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2CreateStatusPageIncidentParams {
    pub status_pages_create_status_page_incident_payload_v2:
        models::StatusPagesCreateStatusPageIncidentPayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_create_status_page_incident_update`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2CreateStatusPageIncidentUpdateParams {
    pub status_pages_create_status_page_incident_update_payload_v2:
        models::StatusPagesCreateStatusPageIncidentUpdatePayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_create_status_page_maintenance`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2CreateStatusPageMaintenanceParams {
    pub status_pages_create_status_page_maintenance_payload_v2:
        models::StatusPagesCreateStatusPageMaintenancePayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_create_status_page_maintenance_update`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2CreateStatusPageMaintenanceUpdateParams {
    pub status_pages_create_status_page_maintenance_update_payload_v2:
        models::StatusPagesCreateStatusPageMaintenanceUpdatePayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_create_status_page_retrospective_incident`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2CreateStatusPageRetrospectiveIncidentParams {
    pub status_pages_create_status_page_retrospective_incident_payload_v2:
        models::StatusPagesCreateStatusPageRetrospectiveIncidentPayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_delete_status_page_maintenance`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2DeleteStatusPageMaintenanceParams {
    /// ID of the status page maintenance window
    pub status_page_maintenance_id: String,
}

/// struct for passing parameters to the method [`status_pages_v2_list_status_page_incidents`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ListStatusPageIncidentsParams {
    /// ID of the status page. You can find this by calling the ListStatusPages endpoint.
    pub status_page_id: String,
    /// Filter status page incidents to only those that impacted the specified component. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub component_id: Option<String>,
    /// Filter status page incidents to only those that impacted components in the specified group. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub group_id: Option<String>,
    /// Filter status page incidents to only those that impacted the specified sub-page. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub sub_page_id: Option<String>,
    /// Filter status page incidents to only those that had impacts during or after this time.
    pub start_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Filter status page incidents to only those that had impacts during or before this time.
    pub end_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An record's ID. This endpoint will return a list of records after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`status_pages_v2_list_status_page_maintenances`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ListStatusPageMaintenancesParams {
    /// ID of the status page. You can find this by calling the ListStatusPages endpoint.
    pub status_page_id: String,
    /// Filter status page maintenance windows to only those that impacted the specified component. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub component_id: Option<String>,
    /// Filter status page maintenance windows to only those that impacted components in the specified group. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub group_id: Option<String>,
    /// Filter status page maintenance windows to only those that impacted the specified sub-page. This ID may be found by calling the ShowStatusPageStructure endpoint.
    pub sub_page_id: Option<String>,
    /// Filter status page maintenance windows to only those that had impacts during or after this time.
    pub start_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Filter status page maintenance windows to only those that had impacts during or before this time.
    pub end_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An record's ID. This endpoint will return a list of records after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`status_pages_v2_list_status_pages`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ListStatusPagesParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An record's ID. This endpoint will return a list of records after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`status_pages_v2_show_status_page_component_availability`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ShowStatusPageComponentAvailabilityParams {
    /// Start of the availability window
    pub start_at: chrono::DateTime<chrono::FixedOffset>,
    /// End of the availability window
    pub end_at: chrono::DateTime<chrono::FixedOffset>,
    /// ID of the status page. You can find this by calling the ListStatusPages endpoint.
    pub status_page_id: String,
    /// ID of the component. You can find this by calling the ShowStatusPageStructure endpoint.
    pub component_id: String,
}

/// struct for passing parameters to the method [`status_pages_v2_show_status_page_incident`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ShowStatusPageIncidentParams {
    /// ID of the status page incident
    pub status_page_incident_id: String,
}

/// struct for passing parameters to the method [`status_pages_v2_show_status_page_maintenance`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ShowStatusPageMaintenanceParams {
    /// ID of the status page maintenance window
    pub status_page_maintenance_id: String,
}

/// struct for passing parameters to the method [`status_pages_v2_show_status_page_structure`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2ShowStatusPageStructureParams {
    /// ID of the status page
    pub status_page_id: String,
}

/// struct for passing parameters to the method [`status_pages_v2_update_status_page_incident`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2UpdateStatusPageIncidentParams {
    /// ID of the status page incident
    pub status_page_incident_id: String,
    pub status_pages_update_status_page_incident_payload_v2:
        models::StatusPagesUpdateStatusPageIncidentPayloadV2,
}

/// struct for passing parameters to the method [`status_pages_v2_update_status_page_maintenance`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatusPagesV2UpdateStatusPageMaintenanceParams {
    /// ID of the status page maintenance window
    pub status_page_maintenance_id: String,
    pub status_pages_update_status_page_maintenance_payload_v2:
        models::StatusPagesUpdateStatusPageMaintenancePayloadV2,
}

/// struct for typed errors of method [`status_pages_v2_create_status_page_incident`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2CreateStatusPageIncidentError {
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

/// struct for typed errors of method [`status_pages_v2_create_status_page_incident_update`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2CreateStatusPageIncidentUpdateError {
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

/// struct for typed errors of method [`status_pages_v2_create_status_page_maintenance`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2CreateStatusPageMaintenanceError {
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

/// struct for typed errors of method [`status_pages_v2_create_status_page_maintenance_update`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2CreateStatusPageMaintenanceUpdateError {
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

/// struct for typed errors of method [`status_pages_v2_create_status_page_retrospective_incident`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2CreateStatusPageRetrospectiveIncidentError {
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

/// struct for typed errors of method [`status_pages_v2_delete_status_page_maintenance`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2DeleteStatusPageMaintenanceError {
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

/// struct for typed errors of method [`status_pages_v2_list_status_page_incidents`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ListStatusPageIncidentsError {
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

/// struct for typed errors of method [`status_pages_v2_list_status_page_maintenances`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ListStatusPageMaintenancesError {
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

/// struct for typed errors of method [`status_pages_v2_list_status_pages`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ListStatusPagesError {
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

/// struct for typed errors of method [`status_pages_v2_show_status_page_component_availability`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ShowStatusPageComponentAvailabilityError {
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

/// struct for typed errors of method [`status_pages_v2_show_status_page_incident`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ShowStatusPageIncidentError {
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

/// struct for typed errors of method [`status_pages_v2_show_status_page_maintenance`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ShowStatusPageMaintenanceError {
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

/// struct for typed errors of method [`status_pages_v2_show_status_page_structure`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2ShowStatusPageStructureError {
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

/// struct for typed errors of method [`status_pages_v2_update_status_page_incident`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2UpdateStatusPageIncidentError {
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

/// struct for typed errors of method [`status_pages_v2_update_status_page_maintenance`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusPagesV2UpdateStatusPageMaintenanceError {
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

/// Create a status page incident.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_create_status_page_incident(
    configuration: &configuration::Configuration,
    params: StatusPagesV2CreateStatusPageIncidentParams,
) -> Result<
    models::StatusPagesCreateStatusPageIncidentResultV2,
    Error<StatusPagesV2CreateStatusPageIncidentError>,
> {
    let uri_str = format!("{}/v2/status_page_incidents", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.status_pages_create_status_page_incident_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesCreateStatusPageIncidentResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesCreateStatusPageIncidentResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2CreateStatusPageIncidentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Post an update on a Status Page incident.  This is the endpoint to use when resolving an incident - set incident_status to \"resolved\" to end the incident. There is a limit of 100 updates per incident.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_create_status_page_incident_update(
    configuration: &configuration::Configuration,
    params: StatusPagesV2CreateStatusPageIncidentUpdateParams,
) -> Result<
    models::StatusPagesCreateStatusPageIncidentUpdateResultV2,
    Error<StatusPagesV2CreateStatusPageIncidentUpdateError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_incident_updates",
        configuration.base_path
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
    req_builder =
        req_builder.json(&params.status_pages_create_status_page_incident_update_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesCreateStatusPageIncidentUpdateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesCreateStatusPageIncidentUpdateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2CreateStatusPageIncidentUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Schedule a Status Page maintenance window.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_create_status_page_maintenance(
    configuration: &configuration::Configuration,
    params: StatusPagesV2CreateStatusPageMaintenanceParams,
) -> Result<
    models::StatusPagesCreateStatusPageMaintenanceResultV2,
    Error<StatusPagesV2CreateStatusPageMaintenanceError>,
> {
    let uri_str = format!("{}/v2/status_page_maintenances", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.status_pages_create_status_page_maintenance_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesCreateStatusPageMaintenanceResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesCreateStatusPageMaintenanceResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2CreateStatusPageMaintenanceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Post an update on a Status Page maintenance window.  This is the endpoint to use when completing a maintenance window - set maintenance_status to \"maintenance_complete\" to end the maintenance. There is a limit of 100 updates per maintenance window.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_create_status_page_maintenance_update(
    configuration: &configuration::Configuration,
    params: StatusPagesV2CreateStatusPageMaintenanceUpdateParams,
) -> Result<
    models::StatusPagesCreateStatusPageMaintenanceUpdateResultV2,
    Error<StatusPagesV2CreateStatusPageMaintenanceUpdateError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_maintenance_updates",
        configuration.base_path
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
    req_builder =
        req_builder.json(&params.status_pages_create_status_page_maintenance_update_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesCreateStatusPageMaintenanceUpdateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesCreateStatusPageMaintenanceUpdateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2CreateStatusPageMaintenanceUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a retrospective (historical) status page incident.  Use this to backfill a completed incident with a reconstructed timeline of past updates, for example when migrating from another status page provider. Every update's published_at must be in the past, the updates must be ordered chronologically (earliest first), and the final update must set incident_status to \"resolved\".  Retrospective incidents never notify subscribers.  As this endpoint is intended for bulk historical backfill, it has a dedicated rate limit of 5 requests per second (with a burst allowance of 300 requests) per API key. If you exceed it you'll receive a 429 response with a Retry-After header; back off and retry to resume your backfill.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_create_status_page_retrospective_incident(
    configuration: &configuration::Configuration,
    params: StatusPagesV2CreateStatusPageRetrospectiveIncidentParams,
) -> Result<
    models::StatusPagesCreateStatusPageRetrospectiveIncidentResultV2,
    Error<StatusPagesV2CreateStatusPageRetrospectiveIncidentError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_retrospective_incidents",
        configuration.base_path
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
    req_builder =
        req_builder.json(&params.status_pages_create_status_page_retrospective_incident_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesCreateStatusPageRetrospectiveIncidentResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesCreateStatusPageRetrospectiveIncidentResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2CreateStatusPageRetrospectiveIncidentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a Status Page maintenance window.  The maintenance window and its updates stop appearing on your public status page, links to it stop working, and a window that has not yet run publishes no further automated updates. This cannot be undone.  Subscribers are not told the window has been deleted, so anyone already emailed about it will have updates for a maintenance window they can no longer find.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_delete_status_page_maintenance(
    configuration: &configuration::Configuration,
    params: StatusPagesV2DeleteStatusPageMaintenanceParams,
) -> Result<(), Error<StatusPagesV2DeleteStatusPageMaintenanceError>> {
    let uri_str = format!(
        "{}/v2/status_page_maintenances/{status_page_maintenance_id}",
        configuration.base_path,
        status_page_maintenance_id = crate::apis::urlencode(params.status_page_maintenance_id)
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
        let entity: Option<StatusPagesV2DeleteStatusPageMaintenanceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List status page incidents.  This endpoint requires a valid API key but no specific scopes.
pub async fn status_pages_v2_list_status_page_incidents(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ListStatusPageIncidentsParams,
) -> Result<
    models::StatusPagesListStatusPageIncidentsResultV2,
    Error<StatusPagesV2ListStatusPageIncidentsError>,
> {
    let uri_str = format!("{}/v2/status_page_incidents", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("status_page_id", &params.status_page_id.to_string())]);
    if let Some(ref param_value) = params.component_id {
        req_builder = req_builder.query(&[("component_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.group_id {
        req_builder = req_builder.query(&[("group_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sub_page_id {
        req_builder = req_builder.query(&[("sub_page_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.start_at {
        req_builder = req_builder.query(&[("start_at", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_at {
        req_builder = req_builder.query(&[("end_at", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesListStatusPageIncidentsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesListStatusPageIncidentsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ListStatusPageIncidentsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List status page maintenances.  This endpoint requires a valid API key but no specific scopes.
pub async fn status_pages_v2_list_status_page_maintenances(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ListStatusPageMaintenancesParams,
) -> Result<
    models::StatusPagesListStatusPageMaintenancesResultV2,
    Error<StatusPagesV2ListStatusPageMaintenancesError>,
> {
    let uri_str = format!("{}/v2/status_page_maintenances", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("status_page_id", &params.status_page_id.to_string())]);
    if let Some(ref param_value) = params.component_id {
        req_builder = req_builder.query(&[("component_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.group_id {
        req_builder = req_builder.query(&[("group_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sub_page_id {
        req_builder = req_builder.query(&[("sub_page_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.start_at {
        req_builder = req_builder.query(&[("start_at", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_at {
        req_builder = req_builder.query(&[("end_at", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesListStatusPageMaintenancesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesListStatusPageMaintenancesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ListStatusPageMaintenancesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all status pages for your organisation.  This endpoint requires a valid API key but no specific scopes. Use this to find status page IDs for use in other endpoints.
pub async fn status_pages_v2_list_status_pages(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ListStatusPagesParams,
) -> Result<models::StatusPagesListStatusPagesResultV2, Error<StatusPagesV2ListStatusPagesError>> {
    let uri_str = format!("{}/v2/status_pages", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesListStatusPagesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesListStatusPagesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ListStatusPagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show availability for a status page component over a time window.  Pass start_at and end_at as RFC3339 timestamps. The window cannot be longer than 366 days. Availability uses the same rules as the public status page: full and partial outages count as downtime, overlapping impacts are merged, and time before we have data for the component is excluded rather than counted as up.  This endpoint requires a valid API key but no specific scopes. Use ListStatusPages and ShowStatusPageStructure to find status page and component IDs.
pub async fn status_pages_v2_show_status_page_component_availability(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ShowStatusPageComponentAvailabilityParams,
) -> Result<
    models::StatusPagesShowStatusPageComponentAvailabilityResultV2,
    Error<StatusPagesV2ShowStatusPageComponentAvailabilityError>,
> {
    let uri_str = format!(
        "{}/v2/status_pages/{status_page_id}/components/{component_id}/availability",
        configuration.base_path,
        status_page_id = crate::apis::urlencode(params.status_page_id),
        component_id = crate::apis::urlencode(params.component_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("start_at", &params.start_at.to_string())]);
    req_builder = req_builder.query(&[("end_at", &params.end_at.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesShowStatusPageComponentAvailabilityResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesShowStatusPageComponentAvailabilityResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ShowStatusPageComponentAvailabilityError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a status page incident.  This endpoint requires a valid API key but no specific scopes.
pub async fn status_pages_v2_show_status_page_incident(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ShowStatusPageIncidentParams,
) -> Result<
    models::StatusPagesShowStatusPageIncidentResultV2,
    Error<StatusPagesV2ShowStatusPageIncidentError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_incidents/{status_page_incident_id}",
        configuration.base_path,
        status_page_incident_id = crate::apis::urlencode(params.status_page_incident_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesShowStatusPageIncidentResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesShowStatusPageIncidentResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ShowStatusPageIncidentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a status page maintenance window.  This endpoint requires a valid API key but no specific scopes.
pub async fn status_pages_v2_show_status_page_maintenance(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ShowStatusPageMaintenanceParams,
) -> Result<
    models::StatusPagesShowStatusPageMaintenanceResultV2,
    Error<StatusPagesV2ShowStatusPageMaintenanceError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_maintenances/{status_page_maintenance_id}",
        configuration.base_path,
        status_page_maintenance_id = crate::apis::urlencode(params.status_page_maintenance_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesShowStatusPageMaintenanceResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesShowStatusPageMaintenanceResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ShowStatusPageMaintenanceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show the structure of a status page.  This endpoint requires a valid API key but no specific scopes. Returns the components and component groups configured on a status page. Use this to find component IDs when specifying affected components for incidents or maintenance windows.
pub async fn status_pages_v2_show_status_page_structure(
    configuration: &configuration::Configuration,
    params: StatusPagesV2ShowStatusPageStructureParams,
) -> Result<
    models::StatusPagesShowStatusPageStructureResultV2,
    Error<StatusPagesV2ShowStatusPageStructureError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_structures/{status_page_id}",
        configuration.base_path,
        status_page_id = crate::apis::urlencode(params.status_page_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesShowStatusPageStructureResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesShowStatusPageStructureResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2ShowStatusPageStructureError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a status page incident.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_update_status_page_incident(
    configuration: &configuration::Configuration,
    params: StatusPagesV2UpdateStatusPageIncidentParams,
) -> Result<
    models::StatusPagesUpdateStatusPageIncidentResultV2,
    Error<StatusPagesV2UpdateStatusPageIncidentError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_incidents/{status_page_incident_id}",
        configuration.base_path,
        status_page_incident_id = crate::apis::urlencode(params.status_page_incident_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.status_pages_update_status_page_incident_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesUpdateStatusPageIncidentResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesUpdateStatusPageIncidentResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2UpdateStatusPageIncidentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the name and scheduled window of a Status Page maintenance window.  The new start_at and end_at apply to every component this maintenance window affects.  This does not publish an update to your status page and does not notify subscribers. Automated updates only move a window forwards, so one that has already started or completed will not move back to an earlier status. To move a status backwards, or to tell subscribers the schedule has changed, post a maintenance update to POST /status_page_maintenance_updates.  This endpoint requires an API key with the \"Create status page incidents, status page maintenance windows, and publish status page updates\" scope.
pub async fn status_pages_v2_update_status_page_maintenance(
    configuration: &configuration::Configuration,
    params: StatusPagesV2UpdateStatusPageMaintenanceParams,
) -> Result<
    models::StatusPagesUpdateStatusPageMaintenanceResultV2,
    Error<StatusPagesV2UpdateStatusPageMaintenanceError>,
> {
    let uri_str = format!(
        "{}/v2/status_page_maintenances/{status_page_maintenance_id}",
        configuration.base_path,
        status_page_maintenance_id = crate::apis::urlencode(params.status_page_maintenance_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.status_pages_update_status_page_maintenance_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusPagesUpdateStatusPageMaintenanceResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusPagesUpdateStatusPageMaintenanceResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StatusPagesV2UpdateStatusPageMaintenanceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl StatusPagesV2CreateStatusPageIncidentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_pages_create_status_page_incident_payload_v2: models::StatusPagesCreateStatusPageIncidentPayloadV2,
    ) -> Self {
        Self {
            status_pages_create_status_page_incident_payload_v2,
        }
    }

    /// Sets `status_pages_create_status_page_incident_payload_v2`.
    #[must_use]
    pub fn set_status_pages_create_status_page_incident_payload_v2(
        mut self,
        value: models::StatusPagesCreateStatusPageIncidentPayloadV2,
    ) -> Self {
        self.status_pages_create_status_page_incident_payload_v2 = value;
        self
    }
}

impl StatusPagesV2CreateStatusPageIncidentUpdateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_pages_create_status_page_incident_update_payload_v2: models::StatusPagesCreateStatusPageIncidentUpdatePayloadV2,
    ) -> Self {
        Self {
            status_pages_create_status_page_incident_update_payload_v2,
        }
    }

    /// Sets `status_pages_create_status_page_incident_update_payload_v2`.
    #[must_use]
    pub fn set_status_pages_create_status_page_incident_update_payload_v2(
        mut self,
        value: models::StatusPagesCreateStatusPageIncidentUpdatePayloadV2,
    ) -> Self {
        self.status_pages_create_status_page_incident_update_payload_v2 = value;
        self
    }
}

impl StatusPagesV2CreateStatusPageMaintenanceParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_pages_create_status_page_maintenance_payload_v2: models::StatusPagesCreateStatusPageMaintenancePayloadV2,
    ) -> Self {
        Self {
            status_pages_create_status_page_maintenance_payload_v2,
        }
    }

    /// Sets `status_pages_create_status_page_maintenance_payload_v2`.
    #[must_use]
    pub fn set_status_pages_create_status_page_maintenance_payload_v2(
        mut self,
        value: models::StatusPagesCreateStatusPageMaintenancePayloadV2,
    ) -> Self {
        self.status_pages_create_status_page_maintenance_payload_v2 = value;
        self
    }
}

impl StatusPagesV2CreateStatusPageMaintenanceUpdateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_pages_create_status_page_maintenance_update_payload_v2: models::StatusPagesCreateStatusPageMaintenanceUpdatePayloadV2,
    ) -> Self {
        Self {
            status_pages_create_status_page_maintenance_update_payload_v2,
        }
    }

    /// Sets `status_pages_create_status_page_maintenance_update_payload_v2`.
    #[must_use]
    pub fn set_status_pages_create_status_page_maintenance_update_payload_v2(
        mut self,
        value: models::StatusPagesCreateStatusPageMaintenanceUpdatePayloadV2,
    ) -> Self {
        self.status_pages_create_status_page_maintenance_update_payload_v2 = value;
        self
    }
}

impl StatusPagesV2CreateStatusPageRetrospectiveIncidentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_pages_create_status_page_retrospective_incident_payload_v2: models::StatusPagesCreateStatusPageRetrospectiveIncidentPayloadV2,
    ) -> Self {
        Self {
            status_pages_create_status_page_retrospective_incident_payload_v2,
        }
    }

    /// Sets `status_pages_create_status_page_retrospective_incident_payload_v2`.
    #[must_use]
    pub fn set_status_pages_create_status_page_retrospective_incident_payload_v2(
        mut self,
        value: models::StatusPagesCreateStatusPageRetrospectiveIncidentPayloadV2,
    ) -> Self {
        self.status_pages_create_status_page_retrospective_incident_payload_v2 = value;
        self
    }
}

impl StatusPagesV2DeleteStatusPageMaintenanceParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_maintenance_id: impl Into<String>) -> Self {
        Self {
            status_page_maintenance_id: status_page_maintenance_id.into(),
        }
    }

    /// Sets `status_page_maintenance_id`.
    #[must_use]
    pub fn set_status_page_maintenance_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_maintenance_id = value.into();
        self
    }
}

impl StatusPagesV2ListStatusPageIncidentsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_id: impl Into<String>) -> Self {
        Self {
            status_page_id: status_page_id.into(),
            component_id: None,
            group_id: None,
            sub_page_id: None,
            start_at: None,
            end_at: None,
            page_size: None,
            after: None,
        }
    }

    /// Sets `status_page_id`.
    #[must_use]
    pub fn set_status_page_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_id = value.into();
        self
    }

    /// Sets `component_id`.
    #[must_use]
    pub fn set_component_id(mut self, value: impl Into<String>) -> Self {
        self.component_id = Some(value.into());
        self
    }

    /// Sets `group_id`.
    #[must_use]
    pub fn set_group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Sets `sub_page_id`.
    #[must_use]
    pub fn set_sub_page_id(mut self, value: impl Into<String>) -> Self {
        self.sub_page_id = Some(value.into());
        self
    }

    /// Sets `start_at`.
    #[must_use]
    pub fn set_start_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.start_at = Some(value);
        self
    }

    /// Sets `end_at`.
    #[must_use]
    pub fn set_end_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.end_at = Some(value);
        self
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

impl StatusPagesV2ListStatusPageMaintenancesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_id: impl Into<String>) -> Self {
        Self {
            status_page_id: status_page_id.into(),
            component_id: None,
            group_id: None,
            sub_page_id: None,
            start_at: None,
            end_at: None,
            page_size: None,
            after: None,
        }
    }

    /// Sets `status_page_id`.
    #[must_use]
    pub fn set_status_page_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_id = value.into();
        self
    }

    /// Sets `component_id`.
    #[must_use]
    pub fn set_component_id(mut self, value: impl Into<String>) -> Self {
        self.component_id = Some(value.into());
        self
    }

    /// Sets `group_id`.
    #[must_use]
    pub fn set_group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Sets `sub_page_id`.
    #[must_use]
    pub fn set_sub_page_id(mut self, value: impl Into<String>) -> Self {
        self.sub_page_id = Some(value.into());
        self
    }

    /// Sets `start_at`.
    #[must_use]
    pub fn set_start_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.start_at = Some(value);
        self
    }

    /// Sets `end_at`.
    #[must_use]
    pub fn set_end_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.end_at = Some(value);
        self
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

impl StatusPagesV2ListStatusPagesParams {
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

impl StatusPagesV2ShowStatusPageComponentAvailabilityParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        start_at: chrono::DateTime<chrono::FixedOffset>,
        end_at: chrono::DateTime<chrono::FixedOffset>,
        status_page_id: impl Into<String>,
        component_id: impl Into<String>,
    ) -> Self {
        Self {
            start_at,
            end_at,
            status_page_id: status_page_id.into(),
            component_id: component_id.into(),
        }
    }

    /// Sets `start_at`.
    #[must_use]
    pub fn set_start_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.start_at = value;
        self
    }

    /// Sets `end_at`.
    #[must_use]
    pub fn set_end_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.end_at = value;
        self
    }

    /// Sets `status_page_id`.
    #[must_use]
    pub fn set_status_page_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_id = value.into();
        self
    }

    /// Sets `component_id`.
    #[must_use]
    pub fn set_component_id(mut self, value: impl Into<String>) -> Self {
        self.component_id = value.into();
        self
    }
}

impl StatusPagesV2ShowStatusPageIncidentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_incident_id: impl Into<String>) -> Self {
        Self {
            status_page_incident_id: status_page_incident_id.into(),
        }
    }

    /// Sets `status_page_incident_id`.
    #[must_use]
    pub fn set_status_page_incident_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_incident_id = value.into();
        self
    }
}

impl StatusPagesV2ShowStatusPageMaintenanceParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_maintenance_id: impl Into<String>) -> Self {
        Self {
            status_page_maintenance_id: status_page_maintenance_id.into(),
        }
    }

    /// Sets `status_page_maintenance_id`.
    #[must_use]
    pub fn set_status_page_maintenance_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_maintenance_id = value.into();
        self
    }
}

impl StatusPagesV2ShowStatusPageStructureParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(status_page_id: impl Into<String>) -> Self {
        Self {
            status_page_id: status_page_id.into(),
        }
    }

    /// Sets `status_page_id`.
    #[must_use]
    pub fn set_status_page_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_id = value.into();
        self
    }
}

impl StatusPagesV2UpdateStatusPageIncidentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_page_incident_id: impl Into<String>,
        status_pages_update_status_page_incident_payload_v2: models::StatusPagesUpdateStatusPageIncidentPayloadV2,
    ) -> Self {
        Self {
            status_page_incident_id: status_page_incident_id.into(),
            status_pages_update_status_page_incident_payload_v2,
        }
    }

    /// Sets `status_page_incident_id`.
    #[must_use]
    pub fn set_status_page_incident_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_incident_id = value.into();
        self
    }

    /// Sets `status_pages_update_status_page_incident_payload_v2`.
    #[must_use]
    pub fn set_status_pages_update_status_page_incident_payload_v2(
        mut self,
        value: models::StatusPagesUpdateStatusPageIncidentPayloadV2,
    ) -> Self {
        self.status_pages_update_status_page_incident_payload_v2 = value;
        self
    }
}

impl StatusPagesV2UpdateStatusPageMaintenanceParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        status_page_maintenance_id: impl Into<String>,
        status_pages_update_status_page_maintenance_payload_v2: models::StatusPagesUpdateStatusPageMaintenancePayloadV2,
    ) -> Self {
        Self {
            status_page_maintenance_id: status_page_maintenance_id.into(),
            status_pages_update_status_page_maintenance_payload_v2,
        }
    }

    /// Sets `status_page_maintenance_id`.
    #[must_use]
    pub fn set_status_page_maintenance_id(mut self, value: impl Into<String>) -> Self {
        self.status_page_maintenance_id = value.into();
        self
    }

    /// Sets `status_pages_update_status_page_maintenance_payload_v2`.
    #[must_use]
    pub fn set_status_pages_update_status_page_maintenance_payload_v2(
        mut self,
        value: models::StatusPagesUpdateStatusPageMaintenancePayloadV2,
    ) -> Self {
        self.status_pages_update_status_page_maintenance_payload_v2 = value;
        self
    }
}

impl Default for StatusPagesV2ListStatusPagesParams {
    fn default() -> Self {
        Self::new()
    }
}
