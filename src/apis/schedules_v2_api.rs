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

/// struct for passing parameters to the method [`schedules_v2_create`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2CreateParams {
    pub schedules_create_payload_v2: models::SchedulesCreatePayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_create_override`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2CreateOverrideParams {
    pub schedules_create_override_payload_v2: models::SchedulesCreateOverridePayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_create_schedule_replica`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2CreateScheduleReplicaParams {
    /// The schedule to create a replica for
    pub schedule_id: String,
    pub schedules_create_schedule_replica_payload_v2:
        models::SchedulesCreateScheduleReplicaPayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_create_schedule_sync_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2CreateScheduleSyncRuleParams {
    /// The schedule to create a sync rule for
    pub schedule_id: String,
    pub schedules_create_schedule_sync_rule_payload_v2:
        models::SchedulesCreateScheduleSyncRulePayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_destroy`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2DestroyParams {
    /// Unique internal ID of the schedule
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_destroy_override`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2DestroyOverrideParams {
    /// The override ID to delete
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_destroy_schedule_replica`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2DestroyScheduleReplicaParams {
    /// The parent schedule ID
    pub schedule_id: String,
    /// The replica ID to archive
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_destroy_schedule_sync_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2DestroyScheduleSyncRuleParams {
    /// The parent schedule ID
    pub schedule_id: String,
    /// The sync rule ID to archive
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ListParams {
    /// Note that next_shifts will only be returned when the page size is 25 or lower.
    pub page_size: Option<i64>,
    /// A schedule's ID. This endpoint will return a list of schedules after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`schedules_v2_list_overrides`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ListOverridesParams {
    /// The ID of the schedule to get overrides for.
    pub schedule_id: String,
    /// If set, only return overrides on this rotation.
    pub rotation_id: Option<String>,
    /// If set, only return overrides on this layer.
    pub layer_id: Option<String>,
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An override's ID. This endpoint will return a list of overrides after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`schedules_v2_list_schedule_entries`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ListScheduleEntriesParams {
    /// The ID of the schedule to get entries for.
    pub schedule_id: String,
    /// The start of the window to get entries for. May also carry an opaque pagination cursor previously returned in `pagination_meta.after` — pass it back here unchanged to fetch the next page (leave `entry_window_end` unchanged from the original request).
    pub entry_window_start: Option<String>,
    /// The end of the window to get entries for.
    pub entry_window_end: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// struct for passing parameters to the method [`schedules_v2_list_schedule_replicas`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ListScheduleReplicasParams {
    /// The schedule to list replicas for
    pub schedule_id: String,
}

/// struct for passing parameters to the method [`schedules_v2_list_schedule_sync_rules`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ListScheduleSyncRulesParams {
    /// The schedule to list sync rules for
    pub schedule_id: String,
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// A sync rule's ID. This endpoint will return a list of sync rules after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`schedules_v2_preview_schedule_entries`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2PreviewScheduleEntriesParams {
    /// The ID of the schedule to preview entries for.
    pub id: String,
    pub schedules_preview_schedule_entries_payload_v2:
        models::SchedulesPreviewScheduleEntriesPayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ShowParams {
    /// Unique internal ID of the schedule
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_show_override`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ShowOverrideParams {
    /// The override ID to get
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_show_schedule_replica`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ShowScheduleReplicaParams {
    /// The parent schedule ID
    pub schedule_id: String,
    /// The replica ID to show
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_show_schedule_sync_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2ShowScheduleSyncRuleParams {
    /// The parent schedule ID
    pub schedule_id: String,
    /// The sync rule ID
    pub id: String,
}

/// struct for passing parameters to the method [`schedules_v2_update`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2UpdateParams {
    /// The schedule ID to update.
    pub id: String,
    pub schedules_update_payload_v2: models::SchedulesUpdatePayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_update_override`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2UpdateOverrideParams {
    /// The override ID to update
    pub id: String,
    pub schedules_update_override_payload_v2: models::SchedulesUpdateOverridePayloadV2,
}

/// struct for passing parameters to the method [`schedules_v2_update_schedule_sync_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SchedulesV2UpdateScheduleSyncRuleParams {
    /// The parent schedule ID
    pub schedule_id: String,
    /// The sync rule ID
    pub id: String,
    pub schedules_update_schedule_sync_rule_payload_v2:
        models::SchedulesUpdateScheduleSyncRulePayloadV2,
}

/// struct for typed errors of method [`schedules_v2_create`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2CreateError {
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

/// struct for typed errors of method [`schedules_v2_create_override`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2CreateOverrideError {
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

/// struct for typed errors of method [`schedules_v2_create_schedule_replica`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2CreateScheduleReplicaError {
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

/// struct for typed errors of method [`schedules_v2_create_schedule_sync_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2CreateScheduleSyncRuleError {
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

/// struct for typed errors of method [`schedules_v2_destroy`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2DestroyError {
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

/// struct for typed errors of method [`schedules_v2_destroy_override`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2DestroyOverrideError {
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

/// struct for typed errors of method [`schedules_v2_destroy_schedule_replica`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2DestroyScheduleReplicaError {
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

/// struct for typed errors of method [`schedules_v2_destroy_schedule_sync_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2DestroyScheduleSyncRuleError {
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

/// struct for typed errors of method [`schedules_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ListError {
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

/// struct for typed errors of method [`schedules_v2_list_overrides`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ListOverridesError {
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

/// struct for typed errors of method [`schedules_v2_list_schedule_entries`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ListScheduleEntriesError {
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

/// struct for typed errors of method [`schedules_v2_list_schedule_replicas`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ListScheduleReplicasError {
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

/// struct for typed errors of method [`schedules_v2_list_schedule_sync_rules`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ListScheduleSyncRulesError {
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

/// struct for typed errors of method [`schedules_v2_preview_schedule_entries`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2PreviewScheduleEntriesError {
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

/// struct for typed errors of method [`schedules_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ShowError {
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

/// struct for typed errors of method [`schedules_v2_show_override`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ShowOverrideError {
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

/// struct for typed errors of method [`schedules_v2_show_schedule_replica`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ShowScheduleReplicaError {
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

/// struct for typed errors of method [`schedules_v2_show_schedule_sync_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2ShowScheduleSyncRuleError {
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

/// struct for typed errors of method [`schedules_v2_update`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2UpdateError {
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

/// struct for typed errors of method [`schedules_v2_update_override`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2UpdateOverrideError {
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

/// struct for typed errors of method [`schedules_v2_update_schedule_sync_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchedulesV2UpdateScheduleSyncRuleError {
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

/// Create a new schedule.
pub async fn schedules_v2_create(
    configuration: &configuration::Configuration,
    params: SchedulesV2CreateParams,
) -> Result<models::SchedulesCreateResultV2, Error<SchedulesV2CreateError>> {
    let uri_str = format!("{}/v2/schedules", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.schedules_create_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesCreateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesCreateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2CreateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2CreateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new schedule override.
pub async fn schedules_v2_create_override(
    configuration: &configuration::Configuration,
    params: SchedulesV2CreateOverrideParams,
) -> Result<models::SchedulesCreateOverrideResultV2, Error<SchedulesV2CreateOverrideError>> {
    let uri_str = format!("{}/v2/schedule_overrides", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.schedules_create_override_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesCreateOverrideResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesCreateOverrideResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2CreateOverrideError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2CreateOverrideError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new schedule replica.
pub async fn schedules_v2_create_schedule_replica(
    configuration: &configuration::Configuration,
    params: SchedulesV2CreateScheduleReplicaParams,
) -> Result<
    models::SchedulesCreateScheduleReplicaResultV2,
    Error<SchedulesV2CreateScheduleReplicaError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/replicas",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id)
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
    req_builder = req_builder.json(&params.schedules_create_schedule_replica_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesCreateScheduleReplicaResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesCreateScheduleReplicaResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2CreateScheduleReplicaError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2CreateScheduleReplicaError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new sync rule linking a schedule to a sync target.
pub async fn schedules_v2_create_schedule_sync_rule(
    configuration: &configuration::Configuration,
    params: SchedulesV2CreateScheduleSyncRuleParams,
) -> Result<
    models::SchedulesCreateScheduleSyncRuleResultV2,
    Error<SchedulesV2CreateScheduleSyncRuleError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/sync_rules",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id)
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
    req_builder = req_builder.json(&params.schedules_create_schedule_sync_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesCreateScheduleSyncRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesCreateScheduleSyncRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2CreateScheduleSyncRuleError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2CreateScheduleSyncRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archives a single schedule. Will fail if the schedule has active replicas — remove all replicas before deleting.
pub async fn schedules_v2_destroy(
    configuration: &configuration::Configuration,
    params: SchedulesV2DestroyParams,
) -> Result<(), Error<SchedulesV2DestroyError>> {
    let uri_str = format!(
        "{}/v2/schedules/{id}",
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
        let entity: Option<SchedulesV2DestroyError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2DestroyError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a schedule override, restoring whoever the rotations put on-call for that window.  Deleting an override that has already been deleted succeeds, so it is safe to retry.
pub async fn schedules_v2_destroy_override(
    configuration: &configuration::Configuration,
    params: SchedulesV2DestroyOverrideParams,
) -> Result<(), Error<SchedulesV2DestroyOverrideError>> {
    let uri_str = format!(
        "{}/v2/schedule_overrides/{id}",
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
        let entity: Option<SchedulesV2DestroyOverrideError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2DestroyOverrideError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archives a single schedule replica, stopping incident.io from syncing on-call shifts to the external provider.  As with disabling mirroring via the UI, this will remove any upcoming overrides that incident.io has created in the external schedule, restoring it to its original state. If multiple replicas target the same external schedule, overrides are only removed when the last replica pointing to that schedule is deleted.  Note: override cleanup is supported for PagerDuty and Jira Service Management. Opsgenie does not support programmatic override deletion, so overrides must be removed manually.
pub async fn schedules_v2_destroy_schedule_replica(
    configuration: &configuration::Configuration,
    params: SchedulesV2DestroyScheduleReplicaParams,
) -> Result<(), Error<SchedulesV2DestroyScheduleReplicaError>> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/replicas/{id}",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id),
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
        let entity: Option<SchedulesV2DestroyScheduleReplicaError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2DestroyScheduleReplicaError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archive a sync rule, unlinking the schedule from the sync target.
pub async fn schedules_v2_destroy_schedule_sync_rule(
    configuration: &configuration::Configuration,
    params: SchedulesV2DestroyScheduleSyncRuleParams,
) -> Result<(), Error<SchedulesV2DestroyScheduleSyncRuleError>> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/sync_rules/{id}",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id),
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
        let entity: Option<SchedulesV2DestroyScheduleSyncRuleError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2DestroyScheduleSyncRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List configured schedules.
pub async fn schedules_v2_list(
    configuration: &configuration::Configuration,
    params: SchedulesV2ListParams,
) -> Result<models::SchedulesListResultV2, Error<SchedulesV2ListError>> {
    let uri_str = format!("{}/v2/schedules", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the overrides on a schedule.  Overrides are one-off changes layered on top of the rotations, such as someone covering a colleague's shift. This returns the overrides themselves: to see the effective schedule with overrides already merged in, use the schedule entries endpoint instead.  Overrides belong to a specific layer of a specific rotation, so you can narrow the results with `rotation_id` and `layer_id`.  Archived overrides are not returned.
pub async fn schedules_v2_list_overrides(
    configuration: &configuration::Configuration,
    params: SchedulesV2ListOverridesParams,
) -> Result<models::SchedulesListOverridesResultV2, Error<SchedulesV2ListOverridesError>> {
    let uri_str = format!("{}/v2/schedule_overrides", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("schedule_id", &params.schedule_id.to_string())]);
    if let Some(ref param_value) = params.rotation_id {
        req_builder = req_builder.query(&[("rotation_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.layer_id {
        req_builder = req_builder.query(&[("layer_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesListOverridesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesListOverridesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ListOverridesError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2ListOverridesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the schedule entries for a schedule over a window of time.  Use this endpoint to find out who is on-call for a schedule, either right now or at any point in the future. Common uses include:  - Building a calendar or timeline view of who is on-call. - Looking up who was on-call at a particular moment (for example, when an   incident fired). - Exporting upcoming shifts into another system, such as a payroll or   scheduling tool.  The response groups entries into three lists: `scheduled` (the entries the rotation rules produce, before any overrides), `overrides` (any one-off overrides that apply in the window) and `final` (the effective schedule after overrides have been merged in — this is normally the list you want).  Each entry includes the `rotation_id` and `layer_id` it belongs to. Schedules can be made up of multiple rotations (for example, a primary and a secondary rotation) and each rotation can have several layers, and we return entries for every rotation and layer on the schedule.  The endpoint returns all entries that overlap with the given window. If no window is provided we default to a sensible range starting from now.  ## Pagination  Responses are paginated. When more entries are available than fit on a single page, the response includes a `pagination_meta` block with two fields:  - `after_url` — a fully-formed URL for the next page. The simplest way   to paginate is to keep following this URL until it is no longer present. - `after` — an opaque cursor token. To fetch the next page manually,   re-issue the request with `entry_window_start` set to this value and   `entry_window_end` left unchanged from the original request. Treat   the token as opaque — do not parse or modify it.  Keep paginating until `pagination_meta` is absent from the response, at which point you have received every entry in the window.
pub async fn schedules_v2_list_schedule_entries(
    configuration: &configuration::Configuration,
    params: SchedulesV2ListScheduleEntriesParams,
) -> Result<models::SchedulesListScheduleEntriesResultV2, Error<SchedulesV2ListScheduleEntriesError>>
{
    let uri_str = format!("{}/v2/schedule_entries", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("schedule_id", &params.schedule_id.to_string())]);
    if let Some(ref param_value) = params.entry_window_start {
        req_builder = req_builder.query(&[("entry_window_start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.entry_window_end {
        req_builder = req_builder.query(&[("entry_window_end", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesListScheduleEntriesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesListScheduleEntriesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ListScheduleEntriesError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2ListScheduleEntriesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all replicas for a schedule.
pub async fn schedules_v2_list_schedule_replicas(
    configuration: &configuration::Configuration,
    params: SchedulesV2ListScheduleReplicasParams,
) -> Result<
    models::SchedulesListScheduleReplicasResultV2,
    Error<SchedulesV2ListScheduleReplicasError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/replicas",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesListScheduleReplicasResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesListScheduleReplicasResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ListScheduleReplicasError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2ListScheduleReplicasError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the sync rules configured on this schedule.
pub async fn schedules_v2_list_schedule_sync_rules(
    configuration: &configuration::Configuration,
    params: SchedulesV2ListScheduleSyncRulesParams,
) -> Result<
    models::SchedulesListScheduleSyncRulesResultV2,
    Error<SchedulesV2ListScheduleSyncRulesError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/sync_rules",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id)
    );
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesListScheduleSyncRulesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesListScheduleSyncRulesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ListScheduleSyncRulesError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2ListScheduleSyncRulesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Preview the schedule entries that would be generated by a proposed schedule configuration.  Use this endpoint before updating a schedule to see who would be on-call if you saved the supplied schedule payload. The request body uses the same `schedule` payload shape as Update schedule, so you can send the configuration you intend to save without persisting it.  The response uses the same `schedule_entries` envelope as List schedule entries: `scheduled` contains entries produced by the rotation rules, `overrides` contains matching overrides, and `final` contains the effective schedule after overrides are applied.  The preview window is bounded to keep requests predictable. If you ask for more than 91 days, the response is capped to 91 days from `entry_window_start`.
pub async fn schedules_v2_preview_schedule_entries(
    configuration: &configuration::Configuration,
    params: SchedulesV2PreviewScheduleEntriesParams,
) -> Result<
    models::SchedulesPreviewScheduleEntriesResultV2,
    Error<SchedulesV2PreviewScheduleEntriesError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{id}/actions/preview_entries",
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
    req_builder = req_builder.json(&params.schedules_preview_schedule_entries_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesPreviewScheduleEntriesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesPreviewScheduleEntriesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2PreviewScheduleEntriesError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2PreviewScheduleEntriesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single schedule.
pub async fn schedules_v2_show(
    configuration: &configuration::Configuration,
    params: SchedulesV2ShowParams,
) -> Result<models::SchedulesShowResultV2, Error<SchedulesV2ShowError>> {
    let uri_str = format!(
        "{}/v2/schedules/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single schedule override.  Overrides that have been deleted are not returned.  An override's ID is not stable across edits: updating an override that overlaps its neighbours replaces them with new overrides, so an ID you are holding can stop resolving. List the schedule's overrides again to pick up the replacements.
pub async fn schedules_v2_show_override(
    configuration: &configuration::Configuration,
    params: SchedulesV2ShowOverrideParams,
) -> Result<models::SchedulesShowOverrideResultV2, Error<SchedulesV2ShowOverrideError>> {
    let uri_str = format!(
        "{}/v2/schedule_overrides/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesShowOverrideResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesShowOverrideResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ShowOverrideError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2ShowOverrideError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single schedule replica.
pub async fn schedules_v2_show_schedule_replica(
    configuration: &configuration::Configuration,
    params: SchedulesV2ShowScheduleReplicaParams,
) -> Result<models::SchedulesShowScheduleReplicaResultV2, Error<SchedulesV2ShowScheduleReplicaError>>
{
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/replicas/{id}",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id),
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesShowScheduleReplicaResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesShowScheduleReplicaResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ShowScheduleReplicaError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2ShowScheduleReplicaError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single sync rule for a schedule.
pub async fn schedules_v2_show_schedule_sync_rule(
    configuration: &configuration::Configuration,
    params: SchedulesV2ShowScheduleSyncRuleParams,
) -> Result<
    models::SchedulesShowScheduleSyncRuleResultV2,
    Error<SchedulesV2ShowScheduleSyncRuleError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/sync_rules/{id}",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id),
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesShowScheduleSyncRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesShowScheduleSyncRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2ShowScheduleSyncRuleError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2ShowScheduleSyncRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a schedule.  Updating a schedule replaces its entire configuration with the one you send, including any scheduled future versions of its rotations — fetch the schedule first and include every version you want to keep.  To change who's in a rotation from a future date without affecting shifts before then, keep the current version unchanged and add another version of the same rotation with `effective_from` set to when the change should land — ideally an upcoming handover, so nobody is swapped mid-shift. You can check the effect of any configuration before saving it with Preview schedule entries.
pub async fn schedules_v2_update(
    configuration: &configuration::Configuration,
    params: SchedulesV2UpdateParams,
) -> Result<models::SchedulesUpdateResultV2, Error<SchedulesV2UpdateError>> {
    let uri_str = format!(
        "{}/v2/schedules/{id}",
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
    req_builder = req_builder.json(&params.schedules_update_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesUpdateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesUpdateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2UpdateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2UpdateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a schedule override, moving its window or the rotation and layer it sits on.  An override cannot be reassigned: send the user it already has, and delete and recreate it to put someone else on call for that window.  Overrides on a layer cannot overlap, so widening one over its neighbour's window takes that time over. The neighbour is deleted, and any of its time left outside the new window comes back as new overrides with new IDs, so other override IDs on this layer may stop resolving — list the schedule's overrides again to pick up the replacements.
pub async fn schedules_v2_update_override(
    configuration: &configuration::Configuration,
    params: SchedulesV2UpdateOverrideParams,
) -> Result<models::SchedulesUpdateOverrideResultV2, Error<SchedulesV2UpdateOverrideError>> {
    let uri_str = format!(
        "{}/v2/schedule_overrides/{id}",
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
    req_builder = req_builder.json(&params.schedules_update_override_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesUpdateOverrideResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesUpdateOverrideResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2UpdateOverrideError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| SchedulesV2UpdateOverrideError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a sync rule's sync_type and permanent members in place. If the rule's sync target is shared with other schedules, a sync_type change propagates to every linked schedule and the entire operation aborts if the caller lacks edit permission on any of them. Permanent members are scoped to this rule and never propagate.
pub async fn schedules_v2_update_schedule_sync_rule(
    configuration: &configuration::Configuration,
    params: SchedulesV2UpdateScheduleSyncRuleParams,
) -> Result<
    models::SchedulesUpdateScheduleSyncRuleResultV2,
    Error<SchedulesV2UpdateScheduleSyncRuleError>,
> {
    let uri_str = format!(
        "{}/v2/schedules/{schedule_id}/sync_rules/{id}",
        configuration.base_path,
        schedule_id = crate::apis::urlencode(params.schedule_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.schedules_update_schedule_sync_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SchedulesUpdateScheduleSyncRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SchedulesUpdateScheduleSyncRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SchedulesV2UpdateScheduleSyncRuleError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| SchedulesV2UpdateScheduleSyncRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl SchedulesV2CreateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedules_create_payload_v2: models::SchedulesCreatePayloadV2) -> Self {
        Self {
            schedules_create_payload_v2,
        }
    }

    /// Sets `schedules_create_payload_v2`.
    #[must_use]
    pub fn set_schedules_create_payload_v2(
        mut self,
        value: models::SchedulesCreatePayloadV2,
    ) -> Self {
        self.schedules_create_payload_v2 = value;
        self
    }
}

impl SchedulesV2CreateOverrideParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        schedules_create_override_payload_v2: models::SchedulesCreateOverridePayloadV2,
    ) -> Self {
        Self {
            schedules_create_override_payload_v2,
        }
    }

    /// Sets `schedules_create_override_payload_v2`.
    #[must_use]
    pub fn set_schedules_create_override_payload_v2(
        mut self,
        value: models::SchedulesCreateOverridePayloadV2,
    ) -> Self {
        self.schedules_create_override_payload_v2 = value;
        self
    }
}

impl SchedulesV2CreateScheduleReplicaParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        schedule_id: impl Into<String>,
        schedules_create_schedule_replica_payload_v2: models::SchedulesCreateScheduleReplicaPayloadV2,
    ) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            schedules_create_schedule_replica_payload_v2,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `schedules_create_schedule_replica_payload_v2`.
    #[must_use]
    pub fn set_schedules_create_schedule_replica_payload_v2(
        mut self,
        value: models::SchedulesCreateScheduleReplicaPayloadV2,
    ) -> Self {
        self.schedules_create_schedule_replica_payload_v2 = value;
        self
    }
}

impl SchedulesV2CreateScheduleSyncRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        schedule_id: impl Into<String>,
        schedules_create_schedule_sync_rule_payload_v2: models::SchedulesCreateScheduleSyncRulePayloadV2,
    ) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            schedules_create_schedule_sync_rule_payload_v2,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `schedules_create_schedule_sync_rule_payload_v2`.
    #[must_use]
    pub fn set_schedules_create_schedule_sync_rule_payload_v2(
        mut self,
        value: models::SchedulesCreateScheduleSyncRulePayloadV2,
    ) -> Self {
        self.schedules_create_schedule_sync_rule_payload_v2 = value;
        self
    }
}

impl SchedulesV2DestroyParams {
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

impl SchedulesV2DestroyOverrideParams {
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

impl SchedulesV2DestroyScheduleReplicaParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            id: id.into(),
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl SchedulesV2DestroyScheduleSyncRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            id: id.into(),
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl SchedulesV2ListParams {
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

impl SchedulesV2ListOverridesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            rotation_id: None,
            layer_id: None,
            page_size: None,
            after: None,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `rotation_id`.
    #[must_use]
    pub fn set_rotation_id(mut self, value: impl Into<String>) -> Self {
        self.rotation_id = Some(value.into());
        self
    }

    /// Sets `layer_id`.
    #[must_use]
    pub fn set_layer_id(mut self, value: impl Into<String>) -> Self {
        self.layer_id = Some(value.into());
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

impl SchedulesV2ListScheduleEntriesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            entry_window_start: None,
            entry_window_end: None,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `entry_window_start`.
    #[must_use]
    pub fn set_entry_window_start(mut self, value: impl Into<String>) -> Self {
        self.entry_window_start = Some(value.into());
        self
    }

    /// Sets `entry_window_end`.
    #[must_use]
    pub fn set_entry_window_end(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.entry_window_end = Some(value);
        self
    }
}

impl SchedulesV2ListScheduleReplicasParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }
}

impl SchedulesV2ListScheduleSyncRulesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            page_size: None,
            after: None,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
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

impl SchedulesV2PreviewScheduleEntriesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        schedules_preview_schedule_entries_payload_v2: models::SchedulesPreviewScheduleEntriesPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            schedules_preview_schedule_entries_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `schedules_preview_schedule_entries_payload_v2`.
    #[must_use]
    pub fn set_schedules_preview_schedule_entries_payload_v2(
        mut self,
        value: models::SchedulesPreviewScheduleEntriesPayloadV2,
    ) -> Self {
        self.schedules_preview_schedule_entries_payload_v2 = value;
        self
    }
}

impl SchedulesV2ShowParams {
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

impl SchedulesV2ShowOverrideParams {
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

impl SchedulesV2ShowScheduleReplicaParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            id: id.into(),
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl SchedulesV2ShowScheduleSyncRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(schedule_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            id: id.into(),
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl SchedulesV2UpdateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        schedules_update_payload_v2: models::SchedulesUpdatePayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            schedules_update_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `schedules_update_payload_v2`.
    #[must_use]
    pub fn set_schedules_update_payload_v2(
        mut self,
        value: models::SchedulesUpdatePayloadV2,
    ) -> Self {
        self.schedules_update_payload_v2 = value;
        self
    }
}

impl SchedulesV2UpdateOverrideParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        schedules_update_override_payload_v2: models::SchedulesUpdateOverridePayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            schedules_update_override_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `schedules_update_override_payload_v2`.
    #[must_use]
    pub fn set_schedules_update_override_payload_v2(
        mut self,
        value: models::SchedulesUpdateOverridePayloadV2,
    ) -> Self {
        self.schedules_update_override_payload_v2 = value;
        self
    }
}

impl SchedulesV2UpdateScheduleSyncRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        schedule_id: impl Into<String>,
        id: impl Into<String>,
        schedules_update_schedule_sync_rule_payload_v2: models::SchedulesUpdateScheduleSyncRulePayloadV2,
    ) -> Self {
        Self {
            schedule_id: schedule_id.into(),
            id: id.into(),
            schedules_update_schedule_sync_rule_payload_v2,
        }
    }

    /// Sets `schedule_id`.
    #[must_use]
    pub fn set_schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `schedules_update_schedule_sync_rule_payload_v2`.
    #[must_use]
    pub fn set_schedules_update_schedule_sync_rule_payload_v2(
        mut self,
        value: models::SchedulesUpdateScheduleSyncRulePayloadV2,
    ) -> Self {
        self.schedules_update_schedule_sync_rule_payload_v2 = value;
        self
    }
}

impl Default for SchedulesV2ListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulesV2CreateError {
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

impl SchedulesV2CreateOverrideError {
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

impl SchedulesV2CreateScheduleReplicaError {
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

impl SchedulesV2CreateScheduleSyncRuleError {
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

impl SchedulesV2DestroyError {
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

impl SchedulesV2DestroyOverrideError {
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

impl SchedulesV2DestroyScheduleReplicaError {
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

impl SchedulesV2DestroyScheduleSyncRuleError {
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

impl SchedulesV2ListError {
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

impl SchedulesV2ListOverridesError {
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

impl SchedulesV2ListScheduleEntriesError {
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

impl SchedulesV2ListScheduleReplicasError {
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

impl SchedulesV2ListScheduleSyncRulesError {
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

impl SchedulesV2PreviewScheduleEntriesError {
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

impl SchedulesV2ShowError {
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

impl SchedulesV2ShowOverrideError {
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

impl SchedulesV2ShowScheduleReplicaError {
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

impl SchedulesV2ShowScheduleSyncRuleError {
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

impl SchedulesV2UpdateError {
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

impl SchedulesV2UpdateOverrideError {
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

impl SchedulesV2UpdateScheduleSyncRuleError {
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
