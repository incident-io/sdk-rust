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
pub struct AnnouncementRulesUpdatePayloadV2 {
    /// Incidents are announced when they match any of these condition groups
    #[serde(rename = "condition_groups")]
    pub condition_groups: Vec<models::ConditionGroupPayloadV2>,
    /// Whether to remove announcement posts when the incident no longer matches this rule's conditions. Defaults to leaving them in place.
    #[serde(
        rename = "conditions_no_longer_apply_behaviour",
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions_no_longer_apply_behaviour: Option<ConditionsNoLongerApplyBehaviour>,
    /// Microsoft Teams channels to post announcements into, as team_id/channel_id. Required when the organisation uses Microsoft Teams.
    #[serde(
        rename = "microsoft_teams_channel_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub microsoft_teams_channel_ids: Option<Vec<String>>,
    /// Which incidents are announced: live incidents only, or triage incidents too
    #[serde(rename = "mode")]
    pub mode: Mode,
    /// Human readable name for the rule
    #[serde(rename = "name")]
    pub name: String,
    /// IDs of the teams that own this rule. The existing owning teams are kept when omitted.
    #[serde(rename = "owning_team_ids", skip_serializing_if = "Option::is_none")]
    pub owning_team_ids: Option<Vec<String>>,
    /// Which private incidents this rule announces: every private incident (all), those an owning team can see (owning_teams), or none. Defaults to none on create, and is left unchanged on update when omitted.
    #[serde(
        rename = "private_incident_scope",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_incident_scope: Option<PrivateIncidentScope>,
    /// IDs of the Slack channels to post announcements into. Required when the organisation uses Slack.
    #[serde(rename = "slack_channel_ids", skip_serializing_if = "Option::is_none")]
    pub slack_channel_ids: Option<Vec<String>>,
    /// ID of the announcement template used to render this rule's posts. The existing template is kept when omitted.
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Where incident updates are shared once the incident is announced
    #[serde(rename = "update_sharing_mode")]
    pub update_sharing_mode: UpdateSharingMode,
}

impl AnnouncementRulesUpdatePayloadV2 {
    pub fn new(
        condition_groups: Vec<models::ConditionGroupPayloadV2>,
        mode: Mode,
        name: impl Into<String>,
        update_sharing_mode: UpdateSharingMode,
    ) -> AnnouncementRulesUpdatePayloadV2 {
        AnnouncementRulesUpdatePayloadV2 {
            condition_groups,
            conditions_no_longer_apply_behaviour: None,
            microsoft_teams_channel_ids: None,
            mode,
            name: name.into(),
            owning_team_ids: None,
            private_incident_scope: None,
            slack_channel_ids: None,
            template_id: None,
            update_sharing_mode,
        }
    }
}
/// Whether to remove announcement posts when the incident no longer matches this rule's conditions. Defaults to leaving them in place.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConditionsNoLongerApplyBehaviour {
    #[serde(rename = "leave_in_place")]
    LeaveInPlace,
    #[serde(rename = "remove")]
    Remove,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for ConditionsNoLongerApplyBehaviour {
    fn default() -> ConditionsNoLongerApplyBehaviour {
        Self::LeaveInPlace
    }
}
/// Which incidents are announced: live incidents only, or triage incidents too
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Mode {
    #[serde(rename = "live_and_closed")]
    LiveAndClosed,
    #[serde(rename = "include_triage")]
    IncludeTriage,
    #[serde(rename = "include_triage_and_merged")]
    IncludeTriageAndMerged,
    #[serde(rename = "include_declined_and_merged")]
    IncludeDeclinedAndMerged,
    #[serde(rename = "include_all")]
    IncludeAll,
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
        Self::LiveAndClosed
    }
}
/// Which private incidents this rule announces: every private incident (all), those an owning team can see (owning_teams), or none. Defaults to none on create, and is left unchanged on update when omitted.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PrivateIncidentScope {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "owning_teams")]
    OwningTeams,
    #[serde(rename = "none")]
    None,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for PrivateIncidentScope {
    fn default() -> PrivateIncidentScope {
        Self::All
    }
}
/// Where incident updates are shared once the incident is announced
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UpdateSharingMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thread")]
    Thread,
    #[serde(rename = "thread_and_channel")]
    ThreadAndChannel,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for UpdateSharingMode {
    fn default() -> UpdateSharingMode {
        Self::None
    }
}

// --- generated by scripts/fix_generated.py ---

impl AnnouncementRulesUpdatePayloadV2 {
    /// Sets `condition_groups`.
    #[must_use]
    pub fn set_condition_groups(mut self, value: Vec<models::ConditionGroupPayloadV2>) -> Self {
        self.condition_groups = value;
        self
    }

    /// Sets `conditions_no_longer_apply_behaviour`.
    #[must_use]
    pub fn set_conditions_no_longer_apply_behaviour(
        mut self,
        value: ConditionsNoLongerApplyBehaviour,
    ) -> Self {
        self.conditions_no_longer_apply_behaviour = Some(value);
        self
    }

    /// Sets `microsoft_teams_channel_ids`.
    #[must_use]
    pub fn set_microsoft_teams_channel_ids(mut self, value: Vec<String>) -> Self {
        self.microsoft_teams_channel_ids = Some(value);
        self
    }

    /// Sets `mode`.
    #[must_use]
    pub fn set_mode(mut self, value: Mode) -> Self {
        self.mode = value;
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn set_name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    /// Sets `owning_team_ids`.
    #[must_use]
    pub fn set_owning_team_ids(mut self, value: Vec<String>) -> Self {
        self.owning_team_ids = Some(value);
        self
    }

    /// Sets `private_incident_scope`.
    #[must_use]
    pub fn set_private_incident_scope(mut self, value: PrivateIncidentScope) -> Self {
        self.private_incident_scope = Some(value);
        self
    }

    /// Sets `slack_channel_ids`.
    #[must_use]
    pub fn set_slack_channel_ids(mut self, value: Vec<String>) -> Self {
        self.slack_channel_ids = Some(value);
        self
    }

    /// Sets `template_id`.
    #[must_use]
    pub fn set_template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    /// Sets `update_sharing_mode`.
    #[must_use]
    pub fn set_update_sharing_mode(mut self, value: UpdateSharingMode) -> Self {
        self.update_sharing_mode = value;
        self
    }
}
