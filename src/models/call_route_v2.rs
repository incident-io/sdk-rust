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

/// CallRouteV2 : A call route is a phone number your customers can call to reach whoever is on call, for an urgent support line or a regulator hotline.  When a call comes in we work down the route's path, ringing each level's targets in turn until someone answers, then connect them to the caller. A trailing voicemail node records a message instead. Every call raises an alert, so calls can open incidents through an alert route.  List and edit call routes here. Create and delete them in the dashboard.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallRouteV2 {
    /// The numbers allowed to call this route. Only enforced when use_caller_allowlist is true.
    #[serde(rename = "allowed_callers")]
    pub allowed_callers: Vec<models::CallRouteAllowedCallerV2>,
    /// The country this route's number belongs to
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// Where this route is in provisioning. Only an active route answers calls: * pending: created, and awaiting manual work from us * pending_regulatory_information: awaiting regulatory compliance information, collected in the dashboard * pending_number: compliance is settled, and we're provisioning a number * active: fully provisioned, and answering calls
    #[serde(rename = "current_state")]
    pub current_state: CurrentState,
    /// The language we speak voice prompts in, via text-to-speech
    #[serde(rename = "custom_language")]
    pub custom_language: CustomLanguage,
    /// Unique identifier for this call route
    #[serde(rename = "id")]
    pub id: String,
    /// Name for this call route
    #[serde(rename = "name")]
    pub name: String,
    /// The phone-tree menu this route presents. Empty when callers are routed down the route's path.
    #[serde(rename = "options")]
    pub options: Vec<models::CallRouteOptionV2>,
    /// Who we page when a call comes in. Empty when this route presents a phone-tree menu, in which case each menu option carries its own path.
    #[serde(rename = "path")]
    pub path: Vec<models::CallRoutePathNodeV2>,
    /// The number your customers call to reach this route, once one has been provisioned
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The type of phone number, which determines the regulatory requirements for provisioning it
    #[serde(rename = "phone_number_type", skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<PhoneNumberType>,
    /// Which number responders see when we call them: * route_number: this route's own number * oncall_number: an incident.io on-call number
    #[serde(rename = "responder_caller_id")]
    pub responder_caller_id: ResponderCallerId,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether this route only answers calls from its allowed callers
    #[serde(rename = "use_caller_allowlist")]
    pub use_caller_allowlist: bool,
}

impl CallRouteV2 {
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
/// Where this route is in provisioning. Only an active route answers calls: * pending: created, and awaiting manual work from us * pending_regulatory_information: awaiting regulatory compliance information, collected in the dashboard * pending_number: compliance is settled, and we're provisioning a number * active: fully provisioned, and answering calls
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CurrentState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "pending_regulatory_information")]
    PendingRegulatoryInformation,
    #[serde(rename = "pending_number")]
    PendingNumber,
    #[serde(rename = "active")]
    Active,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for CurrentState {
    fn default() -> CurrentState {
        Self::Pending
    }
}
/// The language we speak voice prompts in, via text-to-speech
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CustomLanguage {
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "nl-NL")]
    NlNl,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for CustomLanguage {
    fn default() -> CustomLanguage {
        Self::EnUs
    }
}
/// The type of phone number, which determines the regulatory requirements for provisioning it
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PhoneNumberType {
    #[serde(rename = "toll_free")]
    TollFree,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "national")]
    National,
    #[serde(rename = "local")]
    Local,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for PhoneNumberType {
    fn default() -> PhoneNumberType {
        Self::TollFree
    }
}
/// Which number responders see when we call them: * route_number: this route's own number * oncall_number: an incident.io on-call number
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ResponderCallerId {
    #[serde(rename = "route_number")]
    RouteNumber,
    #[serde(rename = "oncall_number")]
    OncallNumber,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for ResponderCallerId {
    fn default() -> ResponderCallerId {
        Self::RouteNumber
    }
}

// --- generated by scripts/fix_generated.py ---

impl CallRouteV2 {
    /// Sets `allowed_callers`.
    #[must_use]
    pub fn set_allowed_callers(mut self, value: Vec<models::CallRouteAllowedCallerV2>) -> Self {
        self.allowed_callers = value;
        self
    }

    /// Sets `country_code`.
    #[must_use]
    pub fn set_country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    /// Sets `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.created_at = value;
        self
    }

    /// Sets `current_state`.
    #[must_use]
    pub fn set_current_state(mut self, value: CurrentState) -> Self {
        self.current_state = value;
        self
    }

    /// Sets `custom_language`.
    #[must_use]
    pub fn set_custom_language(mut self, value: CustomLanguage) -> Self {
        self.custom_language = value;
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `name`.
    #[must_use]
    pub fn set_name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    /// Sets `options`.
    #[must_use]
    pub fn set_options(mut self, value: Vec<models::CallRouteOptionV2>) -> Self {
        self.options = value;
        self
    }

    /// Sets `path`.
    #[must_use]
    pub fn set_path(mut self, value: Vec<models::CallRoutePathNodeV2>) -> Self {
        self.path = value;
        self
    }

    /// Sets `phone_number`.
    #[must_use]
    pub fn set_phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    /// Sets `phone_number_type`.
    #[must_use]
    pub fn set_phone_number_type(mut self, value: PhoneNumberType) -> Self {
        self.phone_number_type = Some(value);
        self
    }

    /// Sets `responder_caller_id`.
    #[must_use]
    pub fn set_responder_caller_id(mut self, value: ResponderCallerId) -> Self {
        self.responder_caller_id = value;
        self
    }

    /// Sets `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.updated_at = value;
        self
    }

    /// Sets `use_caller_allowlist`.
    #[must_use]
    pub fn set_use_caller_allowlist(mut self, value: bool) -> Self {
        self.use_caller_allowlist = value;
        self
    }
}
