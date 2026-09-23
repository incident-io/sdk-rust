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
pub struct WorkflowV2 {
    /// Conditions that apply to the workflow trigger
    #[serde(rename = "condition_groups")]
    pub condition_groups: Vec<models::ConditionGroupV2>,
    /// Whether to continue executing the workflow if a step fails
    #[serde(rename = "continue_on_step_error")]
    pub continue_on_step_error: bool,
    #[serde(rename = "delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Box<models::WorkflowDelayV2>>,
    /// Expressions that make variables available in the scope
    #[serde(rename = "expressions")]
    pub expressions: Vec<models::ExpressionV2>,
    /// Folder to display the workflow in
    #[serde(rename = "folder", skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// User-configured form fields available in the workflow scope (manual triggers only)
    #[serde(rename = "form_fields", skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<Vec<models::WorkflowFormFieldV2>>,
    /// Unique identifier for the workflow
    #[serde(rename = "id")]
    pub id: String,
    /// Whether to include private escalations
    #[serde(rename = "include_private_escalations")]
    pub include_private_escalations: bool,
    /// DEPRECATED: use `private_incident_scope` instead. `true` when the workflow runs on private incidents (a `private_incident_scope` of `all` or `owning_teams`), `false` when the scope is `none`.
    #[serde(rename = "include_private_incidents")]
    pub include_private_incidents: bool,
    /// Name provided by the user when creating the workflow
    #[serde(rename = "name")]
    pub name: String,
    /// This workflow will run 'once for' a list of references
    #[serde(rename = "once_for")]
    pub once_for: Vec<models::EngineReferenceV2>,
    /// IDs of the teams that own this workflow
    #[serde(rename = "owning_team_ids", skip_serializing_if = "Option::is_none")]
    pub owning_team_ids: Option<Vec<String>>,
    /// Which private incidents this workflow acts on: every private incident (all), those an owning team can see (owning_teams), or none
    #[serde(rename = "private_incident_scope")]
    pub private_incident_scope: PrivateIncidentScope,
    /// The time from which this workflow will run on incidents
    #[serde(rename = "runs_from", skip_serializing_if = "Option::is_none")]
    pub runs_from: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Which incident modes should this workflow run on? By default, workflows only run on standard incidents, but can also be configured to run on test and retrospective incidents.
    #[serde(rename = "runs_on_incident_modes")]
    pub runs_on_incident_modes: Vec<RunsOnIncidentModes>,
    /// Which incidents should the workflow be applied to?
    #[serde(rename = "runs_on_incidents")]
    pub runs_on_incidents: RunsOnIncidents,
    /// The shortform used to trigger this workflow (only applicable for manual triggers)
    #[serde(rename = "shortform", skip_serializing_if = "Option::is_none")]
    pub shortform: Option<String>,
    /// What state this workflow is in
    #[serde(rename = "state")]
    pub state: State,
    /// Steps that are executed as part of the workflow
    #[serde(rename = "steps")]
    pub steps: Vec<models::StepConfigV2>,
    #[serde(rename = "trigger")]
    pub trigger: Box<models::TriggerSlimV2>,
    /// Revision of the workflow, uniquely identifying it's version
    #[serde(rename = "version")]
    pub version: i64,
}

impl WorkflowV2 {
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
/// Which private incidents this workflow acts on: every private incident (all), those an owning team can see (owning_teams), or none
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
/// Which incident modes should this workflow run on? By default, workflows only run on standard incidents, but can also be configured to run on test and retrospective incidents.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RunsOnIncidentModes {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "retrospective")]
    Retrospective,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for RunsOnIncidentModes {
    fn default() -> RunsOnIncidentModes {
        Self::Standard
    }
}
/// Which incidents should the workflow be applied to?
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RunsOnIncidents {
    #[serde(rename = "newly_created")]
    NewlyCreated,
    #[serde(rename = "newly_created_and_active")]
    NewlyCreatedAndActive,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for RunsOnIncidents {
    fn default() -> RunsOnIncidents {
        Self::NewlyCreated
    }
}
/// What state this workflow is in
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "error")]
    Error,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

// --- generated by scripts/fix_generated.py ---

impl WorkflowV2 {
    /// Sets `condition_groups`.
    #[must_use]
    pub fn set_condition_groups(mut self, value: Vec<models::ConditionGroupV2>) -> Self {
        self.condition_groups = value;
        self
    }

    /// Sets `continue_on_step_error`.
    #[must_use]
    pub fn set_continue_on_step_error(mut self, value: bool) -> Self {
        self.continue_on_step_error = value;
        self
    }

    /// Sets `delay`.
    #[must_use]
    pub fn set_delay(mut self, value: models::WorkflowDelayV2) -> Self {
        self.delay = Some(Box::new(value));
        self
    }

    /// Sets `expressions`.
    #[must_use]
    pub fn set_expressions(mut self, value: Vec<models::ExpressionV2>) -> Self {
        self.expressions = value;
        self
    }

    /// Sets `folder`.
    #[must_use]
    pub fn set_folder(mut self, value: impl Into<String>) -> Self {
        self.folder = Some(value.into());
        self
    }

    /// Sets `form_fields`.
    #[must_use]
    pub fn set_form_fields(mut self, value: Vec<models::WorkflowFormFieldV2>) -> Self {
        self.form_fields = Some(value);
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `include_private_escalations`.
    #[must_use]
    pub fn set_include_private_escalations(mut self, value: bool) -> Self {
        self.include_private_escalations = value;
        self
    }

    /// Sets `include_private_incidents`.
    #[must_use]
    pub fn set_include_private_incidents(mut self, value: bool) -> Self {
        self.include_private_incidents = value;
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn set_name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    /// Sets `once_for`.
    #[must_use]
    pub fn set_once_for(mut self, value: Vec<models::EngineReferenceV2>) -> Self {
        self.once_for = value;
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
        self.private_incident_scope = value;
        self
    }

    /// Sets `runs_from`.
    #[must_use]
    pub fn set_runs_from(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.runs_from = Some(value);
        self
    }

    /// Sets `runs_on_incident_modes`.
    #[must_use]
    pub fn set_runs_on_incident_modes(mut self, value: Vec<RunsOnIncidentModes>) -> Self {
        self.runs_on_incident_modes = value;
        self
    }

    /// Sets `runs_on_incidents`.
    #[must_use]
    pub fn set_runs_on_incidents(mut self, value: RunsOnIncidents) -> Self {
        self.runs_on_incidents = value;
        self
    }

    /// Sets `shortform`.
    #[must_use]
    pub fn set_shortform(mut self, value: impl Into<String>) -> Self {
        self.shortform = Some(value.into());
        self
    }

    /// Sets `state`.
    #[must_use]
    pub fn set_state(mut self, value: State) -> Self {
        self.state = value;
        self
    }

    /// Sets `steps`.
    #[must_use]
    pub fn set_steps(mut self, value: Vec<models::StepConfigV2>) -> Self {
        self.steps = value;
        self
    }

    /// Sets `trigger`.
    #[must_use]
    pub fn set_trigger(mut self, value: models::TriggerSlimV2) -> Self {
        self.trigger = Box::new(value);
        self
    }

    /// Sets `version`.
    #[must_use]
    pub fn set_version(mut self, value: i64) -> Self {
        self.version = value;
        self
    }
}
