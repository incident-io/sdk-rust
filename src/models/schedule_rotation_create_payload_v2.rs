/*
 * incident.io
 *
 * This is the API reference for incident.io.  It documents available API endpoints, provides examples of how to use it, and instructions around things like authentication and error handling.  The API is hosted at:  - https://api.incident.io/  And you will need to create an API key via your [incident.io dashboard](https://app.incident.io/settings/api-keys) to make requests.  # Making requests  Here are the key concepts required to make requests to the incident.io API.  ## Authentication  For all requests made to the incident.io API, you'll need an API key.  To create an API key, head to the incident dashboard and visit [API keys](https://app.incident.io/settings/api-keys). When you create the key, you'll be able to choose what actions it can take for your account: choose carefully, as those roles can only be set when you first create the key. We'll only show you the token once, so make sure you store it somewhere safe.  API keys are global to your incident.io account, and can be managed by anyone who has the right permissions. We display the user that created the API key, and the API key will remain valid if that user becomes deactivated.  Once you have the key, you should make requests to the API that set the `Authorization` request header using a \"Bearer\" authentication scheme:  ``` Authorization: Bearer <YOUR_API_KEY> ```  ## Rate Limits  The incident.io API enforces rate limits to ensure consistent performance for all users.  The default rate limit is 1200 requests/minute per API key. This limit applies to most endpoints across the API.  Limits are token buckets that refill continuously rather than resetting on a fixed window boundary. The default bucket holds 1200 requests and refills at 20 per second, so you can burst up to the full bucket and then sustain 20 requests/second indefinitely. There is no boundary at which your quota resets to full in one step.  Some endpoints have lower rate limits, particularly those that interact with external third-party systems that impose their own limitations. These specific limits vary by endpoint.  ### Rate limit headers  Responses to requests authenticated with an API key carry your current allowance, so you can pace yourself rather than waiting to be throttled:  ``` X-RateLimit-Limit: 60, 1200;window=60, 60;window=60 X-RateLimit-Remaining: 59 X-RateLimit-Used: 1 X-RateLimit-Reset: 1785173199 ```  | Header | Meaning | | --- | --- | | `X-RateLimit-Limit` | The quota that binds this request, followed by every limit that applied and the window it applies over | | `X-RateLimit-Remaining` | Requests you can make right now against the binding limit | | `X-RateLimit-Used` | Requests you have spent against it | | `X-RateLimit-Reset` | Unix timestamp (seconds) at which that limit will be back to full |  More than one limit can apply to a request: your API key's overall limit, and for some endpoints a lower limit of their own. `X-RateLimit-Limit` lists all of them, each with its window, so `1200;window=60` means 1200 requests per minute. Because our limits refill continuously rather than resetting on a boundary, that window is what tells you the rate you can sustain: 1200 per 60 seconds is 20 requests/second indefinitely.  `Remaining`, `Used` and `Reset` describe whichever limit has the least allowance left, since that is the one you will hit first.  `X-RateLimit-Remaining` may lag by a small number of requests under high concurrency, and can move by more than the requests you made, because limits scoped to your whole organisation are shared with your other API keys.  Headers are omitted rather than guessed if we cannot determine your allowance for a request.  ### Exceeding a rate limit  When you exceed a rate limit the API responds with `429 Too Many Requests` and a `Retry-After` header giving the number of seconds to wait:  ``` X-RateLimit-Limit: 1200, 1200;window=60 X-RateLimit-Remaining: 0 X-RateLimit-Used: 1200 X-RateLimit-Reset: 1785173199 Retry-After: 1 ```  Prefer `Retry-After` over `X-RateLimit-Reset` when deciding how long to back off. `Retry-After` is when a single request will succeed; `X-RateLimit-Reset` is the later point at which your whole allowance has returned. It is a duration rather than a timestamp, so it does not depend on your clock agreeing with ours.  The 429 also carries a JSON body with the same information:  ```json {     \"type\": \"too_many_requests\",     \"status\": 429,     \"request_id\": \"b839a403-7704-41c1-bf6a-39a2d68caefa\",     \"rate_limit\": {         \"name\": \"api_key_name\",         \"limit\": 1200,         \"remaining\": 0,         \"retry_after\": \"2025-04-17T11:17:18Z\"     },     \"errors\": [         {             \"code\": \"too_many_requests\",             \"message\": \"Too many requests hit the API too quickly. We recommend an exponential backoff of your requests.\"         }     ] } ```  The response includes: * The name of the API key (`name`) * The bucket limit (`limit`) * The number of requests remaining (`remaining`) * When you can retry requests (`retry_after`), as an RFC3339 timestamp  ## Errors  We use standard HTTP response codes to indicate the status or failure of API requests.  The API response body will be JSON, and contain more detailed information on the nature of the error.  An example error when a request is made without an API key:  ```json {   \"type\": \"authentication_error\",   \"status\": 401,   \"request_id\": \"8e3cc412-b49d-4957-9073-2c19d2c61804\",   \"errors\": [     {       \"code\": \"missing_authorization_material\",       \"message\": \"No authorization material provided in request\"     }   ] } ```  Note that the error:  - Contains the HTTP status (`401`) - References the type of error (`authentication_error`) - Includes a `request_id` that can be provided to incident.io support to help  debug questions with your API request - Provides a list of individual errors, which go into detail about why the error  occurred  The most common error will be a 422 Validation Error, which is returned when the request was rejected due to failing validations.  These errors look like this:  ```json {   \"type\": \"validation_error\",   \"status\": 422,   \"request_id\": \"631766c4-4afd-4803-997c-cd700928fa4b\",   \"errors\": [     {       \"code\": \"is_required\",       \"message\": \"A severity is required to open an incident\",       \"source\": {         \"field\": \"severity_id\"       }     }   ] } ```  This error is caused by not providing a severity identifier, which should be at the `severity_id` field of the request payload. Errors like these can be mapped to forms, should you be integrating with the API from a user-interface.  ## Compatibility  We won't make breaking changes to existing API services or endpoints, but will expect integrators to upgrade themselves to the latest API endpoints within 3 months of us deprecating the old service.  We will make changes that are considered backwards compatible, which include:  - Adding new API endpoints and services - Adding new properties to responses from existing API endpoints - Reordering properties returned from existing API endpoints - Adding optional request parameters to existing API endpoints - Altering the format or length of IDs - Adding new values to enums  It is important that clients are robust to these changes, to ensure reliable integrations.  As an example, if you are generating a client using an openapi-generator, ensure the generated client is configured to support unknown enum values, often configured via the `enumUnknownDefaultCase` parameter.  When breaking changes are unavoidable, we'll create a new service version on a separate path, and run them in parallel.  For example:  - https://api.incident.io/v1/incidents - https://api.incident.io/v2/incidents  For any questions, email support@incident.io.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScheduleRotationCreatePayloadV2 {
    /// When this version of the rotation takes effect. A rotation can appear multiple times in `rotations` with the same `id` to schedule changes ahead of time: each version applies from its `effective_from` until the next version's. Leave it unset on a rotation's first or only version.
    #[serde(rename = "effective_from", skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Determines when shifts change hands and who takes them: the first user in `users` comes on shift at this time, handing over to the next user after each `handovers` interval, cycling through the list — for example, weekly handovers from a Monday 09:00 give week-long shifts that change hands on Mondays at 09:00.
    #[serde(rename = "handover_start_at", skip_serializing_if = "Option::is_none")]
    pub handover_start_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The cadence shifts hand over on. With more than one entry, the intervals apply in turn — for example, one day then three days produces alternating one-day and three-day shifts.
    #[serde(rename = "handovers", skip_serializing_if = "Option::is_none")]
    pub handovers: Option<Vec<models::ScheduleRotationHandoverV2>>,
    /// Unique identifier of the rotation
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "layers", skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<models::ScheduleLayerCreatePayloadV2>>,
    /// Name of the rotation
    #[serde(rename = "name")]
    pub name: String,
    /// Scheduling algorithm to use for this rotation. 'fair' balances workload by considering handover duration, while 'sequential' uses simple round-robin rotation through users. Only applies when you have asymmetric handovers (e.g., 2 days then 5 days).
    #[serde(rename = "scheduling_mode", skip_serializing_if = "Option::is_none")]
    pub scheduling_mode: Option<SchedulingMode>,
    /// The people in the rotation, in the order they take shifts.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::UserReferencePayloadV2>>,
    /// DEPRECATED: Use working_intervals instead.
    #[serde(rename = "working_interval", skip_serializing_if = "Option::is_none")]
    pub working_interval: Option<Vec<models::ScheduleRotationWorkingIntervalCreatePayloadV2>>,
    #[serde(rename = "working_intervals", skip_serializing_if = "Option::is_none")]
    pub working_intervals: Option<Vec<models::ScheduleRotationWorkingIntervalCreatePayloadV2>>,
}

impl ScheduleRotationCreatePayloadV2 {
    pub fn new(name: String) -> ScheduleRotationCreatePayloadV2 {
        ScheduleRotationCreatePayloadV2 {
            effective_from: None,
            handover_start_at: None,
            handovers: None,
            id: None,
            layers: None,
            name,
            scheduling_mode: None,
            users: None,
            working_interval: None,
            working_intervals: None,
        }
    }
}
/// Scheduling algorithm to use for this rotation. 'fair' balances workload by considering handover duration, while 'sequential' uses simple round-robin rotation through users. Only applies when you have asymmetric handovers (e.g., 2 days then 5 days).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchedulingMode {
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "sequential")]
    Sequential,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for SchedulingMode {
    fn default() -> SchedulingMode {
        Self::Fair
    }
}

// --- generated by scripts/fix_generated.py ---

impl ScheduleRotationCreatePayloadV2 {
    /// Sets `effective_from`.
    pub fn set_effective_from(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.effective_from = Some(value);
        self
    }

    /// Sets `handover_start_at`.
    pub fn set_handover_start_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.handover_start_at = Some(value);
        self
    }

    /// Sets `handovers`.
    pub fn set_handovers(mut self, value: Vec<models::ScheduleRotationHandoverV2>) -> Self {
        self.handovers = Some(value);
        self
    }

    /// Sets `id`.
    pub fn set_id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets `layers`.
    pub fn set_layers(mut self, value: Vec<models::ScheduleLayerCreatePayloadV2>) -> Self {
        self.layers = Some(value);
        self
    }

    /// Sets `name`.
    pub fn set_name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Sets `scheduling_mode`.
    pub fn set_scheduling_mode(mut self, value: SchedulingMode) -> Self {
        self.scheduling_mode = Some(value);
        self
    }

    /// Sets `users`.
    pub fn set_users(mut self, value: Vec<models::UserReferencePayloadV2>) -> Self {
        self.users = Some(value);
        self
    }

    /// Sets `working_interval`.
    pub fn set_working_interval(mut self, value: Vec<models::ScheduleRotationWorkingIntervalCreatePayloadV2>) -> Self {
        self.working_interval = Some(value);
        self
    }

    /// Sets `working_intervals`.
    pub fn set_working_intervals(mut self, value: Vec<models::ScheduleRotationWorkingIntervalCreatePayloadV2>) -> Self {
        self.working_intervals = Some(value);
        self
    }

}
