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
pub struct IncidentV2 {
    /// The call URL attached to this incident
    #[serde(rename = "call_url", skip_serializing_if = "Option::is_none")]
    pub call_url: Option<String>,
    /// When the incident was created
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "creator")]
    pub creator: Box<models::ActorV2>,
    /// Custom field entries for this incident
    #[serde(rename = "custom_field_entries")]
    pub custom_field_entries: Vec<models::CustomFieldEntryV2>,
    /// Incident duration metrics and their measurements for this incident
    #[serde(rename = "duration_metrics", skip_serializing_if = "Option::is_none")]
    pub duration_metrics: Option<Vec<models::IncidentDurationMetricWithValueV2>>,
    #[serde(
        rename = "external_issue_reference",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_issue_reference: Option<Box<models::ExternalIssueReferenceV2>>,
    /// If this incident has a debrief attached
    #[serde(rename = "has_debrief", skip_serializing_if = "Option::is_none")]
    pub has_debrief: Option<bool>,
    /// Unique identifier for the incident
    #[serde(rename = "id")]
    pub id: String,
    /// A list of who is assigned to each role for this incident
    #[serde(rename = "incident_role_assignments")]
    pub incident_role_assignments: Vec<models::IncidentRoleAssignmentV2>,
    #[serde(rename = "incident_status")]
    pub incident_status: Box<models::IncidentStatusV2>,
    /// Incident lifecycle events and when they occurred
    #[serde(
        rename = "incident_timestamp_values",
        skip_serializing_if = "Option::is_none"
    )]
    pub incident_timestamp_values: Option<Vec<models::IncidentTimestampWithValueV2>>,
    #[serde(rename = "incident_type", skip_serializing_if = "Option::is_none")]
    pub incident_type: Option<Box<models::IncidentTypeV2>>,
    /// When the incident last recorded 'activity'
    #[serde(rename = "last_activity_at")]
    pub last_activity_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the incident is real, a test, a tutorial, or importing as a retrospective incident
    #[serde(rename = "mode")]
    pub mode: Mode,
    /// URL to link to the Microsoft Teams channel
    #[serde(
        rename = "ms_teams_channel_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub ms_teams_channel_url: Option<String>,
    /// Explanation of the incident
    #[serde(rename = "name")]
    pub name: String,
    /// A permanent link to the homepage for this incident
    #[serde(rename = "permalink", skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    /// An array of IDs of postmortem documents for this incident
    #[serde(
        rename = "postmortem_document_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub postmortem_document_ids: Option<Vec<String>>,
    /// The URL of the incident post-mortem document
    #[serde(
        rename = "postmortem_document_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub postmortem_document_url: Option<String>,
    /// Reference to this incident, as displayed across the product
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<Box<models::SeverityV2>>,
    /// ID of the Slack channel in the organisation Slack workspace. Note that the channel is sometimes created asynchronously, so may not be present when the incident is just created.
    #[serde(rename = "slack_channel_id")]
    pub slack_channel_id: String,
    /// Name of the slack channel
    #[serde(rename = "slack_channel_name", skip_serializing_if = "Option::is_none")]
    pub slack_channel_name: Option<String>,
    /// URL to link to the slack channel
    #[serde(rename = "slack_channel_url", skip_serializing_if = "Option::is_none")]
    pub slack_channel_url: Option<String>,
    /// ID of the Slack team / workspace. This is only required if you are using a Slack Enterprise Grid with multiple teams.
    #[serde(rename = "slack_team_id")]
    pub slack_team_id: String,
    /// Detailed description of the incident
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// IDs of the teams that own this incident, resolved from your team settings. Empty when no teams match.
    #[serde(rename = "team_ids")]
    pub team_ids: Vec<String>,
    /// When the incident was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the incident should be open to anyone in your Slack workspace (public), or invite-only (private). For more information on Private Incidents see our [docs](https://docs.incident.io/incidents/sensitive-incidents).
    #[serde(rename = "visibility")]
    pub visibility: Visibility,
    /// Amount of time spent on the incident in late hours
    #[serde(
        rename = "workload_minutes_late",
        skip_serializing_if = "Option::is_none"
    )]
    pub workload_minutes_late: Option<f64>,
    /// Amount of time spent on the incident in sleeping hours
    #[serde(
        rename = "workload_minutes_sleeping",
        skip_serializing_if = "Option::is_none"
    )]
    pub workload_minutes_sleeping: Option<f64>,
    /// Amount of time spent on the incident in total
    #[serde(
        rename = "workload_minutes_total",
        skip_serializing_if = "Option::is_none"
    )]
    pub workload_minutes_total: Option<f64>,
    /// Amount of time spent on the incident in working hours
    #[serde(
        rename = "workload_minutes_working",
        skip_serializing_if = "Option::is_none"
    )]
    pub workload_minutes_working: Option<f64>,
}

impl IncidentV2 {
    /// A value with every field at its default.
    ///
    /// This is a response type, so you receive one rather than
    /// building it. Set the fields you need with the `set_*`
    /// methods below — deliberately not a required-argument
    /// constructor, because then the schema adding a required
    /// property would change this signature and break you.
    pub fn new() -> Self {
        Default::default()
    }
}
/// Whether the incident is real, a test, a tutorial, or importing as a retrospective incident
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Mode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "retrospective")]
    Retrospective,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "tutorial")]
    Tutorial,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Standard
    }
}
/// Whether the incident should be open to anyone in your Slack workspace (public), or invite-only (private). For more information on Private Incidents see our [docs](https://docs.incident.io/incidents/sensitive-incidents).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}

// --- generated by scripts/fix_generated.py ---

impl IncidentV2 {
    /// Sets `call_url`.
    #[must_use]
    pub fn set_call_url(mut self, value: impl Into<String>) -> Self {
        self.call_url = Some(value.into());
        self
    }

    /// Sets `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.created_at = value;
        self
    }

    /// Sets `creator`.
    #[must_use]
    pub fn set_creator(mut self, value: models::ActorV2) -> Self {
        self.creator = Box::new(value);
        self
    }

    /// Sets `custom_field_entries`.
    #[must_use]
    pub fn set_custom_field_entries(mut self, value: Vec<models::CustomFieldEntryV2>) -> Self {
        self.custom_field_entries = value;
        self
    }

    /// Sets `duration_metrics`.
    #[must_use]
    pub fn set_duration_metrics(
        mut self,
        value: Vec<models::IncidentDurationMetricWithValueV2>,
    ) -> Self {
        self.duration_metrics = Some(value);
        self
    }

    /// Sets `external_issue_reference`.
    #[must_use]
    pub fn set_external_issue_reference(mut self, value: models::ExternalIssueReferenceV2) -> Self {
        self.external_issue_reference = Some(Box::new(value));
        self
    }

    /// Sets `has_debrief`.
    #[must_use]
    pub fn set_has_debrief(mut self, value: bool) -> Self {
        self.has_debrief = Some(value);
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `incident_role_assignments`.
    #[must_use]
    pub fn set_incident_role_assignments(
        mut self,
        value: Vec<models::IncidentRoleAssignmentV2>,
    ) -> Self {
        self.incident_role_assignments = value;
        self
    }

    /// Sets `incident_status`.
    #[must_use]
    pub fn set_incident_status(mut self, value: models::IncidentStatusV2) -> Self {
        self.incident_status = Box::new(value);
        self
    }

    /// Sets `incident_timestamp_values`.
    #[must_use]
    pub fn set_incident_timestamp_values(
        mut self,
        value: Vec<models::IncidentTimestampWithValueV2>,
    ) -> Self {
        self.incident_timestamp_values = Some(value);
        self
    }

    /// Sets `incident_type`.
    #[must_use]
    pub fn set_incident_type(mut self, value: models::IncidentTypeV2) -> Self {
        self.incident_type = Some(Box::new(value));
        self
    }

    /// Sets `last_activity_at`.
    #[must_use]
    pub fn set_last_activity_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.last_activity_at = value;
        self
    }

    /// Sets `mode`.
    #[must_use]
    pub fn set_mode(mut self, value: Mode) -> Self {
        self.mode = value;
        self
    }

    /// Sets `ms_teams_channel_url`.
    #[must_use]
    pub fn set_ms_teams_channel_url(mut self, value: impl Into<String>) -> Self {
        self.ms_teams_channel_url = Some(value.into());
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn set_name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    /// Sets `permalink`.
    #[must_use]
    pub fn set_permalink(mut self, value: impl Into<String>) -> Self {
        self.permalink = Some(value.into());
        self
    }

    /// Sets `postmortem_document_ids`.
    #[must_use]
    pub fn set_postmortem_document_ids(mut self, value: Vec<String>) -> Self {
        self.postmortem_document_ids = Some(value);
        self
    }

    /// Sets `postmortem_document_url`.
    #[must_use]
    pub fn set_postmortem_document_url(mut self, value: impl Into<String>) -> Self {
        self.postmortem_document_url = Some(value.into());
        self
    }

    /// Sets `reference`.
    #[must_use]
    pub fn set_reference(mut self, value: impl Into<String>) -> Self {
        self.reference = value.into();
        self
    }

    /// Sets `severity`.
    #[must_use]
    pub fn set_severity(mut self, value: models::SeverityV2) -> Self {
        self.severity = Some(Box::new(value));
        self
    }

    /// Sets `slack_channel_id`.
    #[must_use]
    pub fn set_slack_channel_id(mut self, value: impl Into<String>) -> Self {
        self.slack_channel_id = value.into();
        self
    }

    /// Sets `slack_channel_name`.
    #[must_use]
    pub fn set_slack_channel_name(mut self, value: impl Into<String>) -> Self {
        self.slack_channel_name = Some(value.into());
        self
    }

    /// Sets `slack_channel_url`.
    #[must_use]
    pub fn set_slack_channel_url(mut self, value: impl Into<String>) -> Self {
        self.slack_channel_url = Some(value.into());
        self
    }

    /// Sets `slack_team_id`.
    #[must_use]
    pub fn set_slack_team_id(mut self, value: impl Into<String>) -> Self {
        self.slack_team_id = value.into();
        self
    }

    /// Sets `summary`.
    #[must_use]
    pub fn set_summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    /// Sets `team_ids`.
    #[must_use]
    pub fn set_team_ids(mut self, value: Vec<String>) -> Self {
        self.team_ids = value;
        self
    }

    /// Sets `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.updated_at = value;
        self
    }

    /// Sets `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Visibility) -> Self {
        self.visibility = value;
        self
    }

    /// Sets `workload_minutes_late`.
    #[must_use]
    pub fn set_workload_minutes_late(mut self, value: f64) -> Self {
        self.workload_minutes_late = Some(value);
        self
    }

    /// Sets `workload_minutes_sleeping`.
    #[must_use]
    pub fn set_workload_minutes_sleeping(mut self, value: f64) -> Self {
        self.workload_minutes_sleeping = Some(value);
        self
    }

    /// Sets `workload_minutes_total`.
    #[must_use]
    pub fn set_workload_minutes_total(mut self, value: f64) -> Self {
        self.workload_minutes_total = Some(value);
        self
    }

    /// Sets `workload_minutes_working`.
    #[must_use]
    pub fn set_workload_minutes_working(mut self, value: f64) -> Self {
        self.workload_minutes_working = Some(value);
        self
    }
}
