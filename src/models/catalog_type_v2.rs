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
pub struct CatalogTypeV2 {
    /// Annotations that can track metadata about this type
    #[serde(rename = "annotations")]
    pub annotations: std::collections::HashMap<String, String>,
    /// What categories is this type considered part of
    #[serde(rename = "categories")]
    pub categories: Vec<Categories>,
    /// Sets the display color of this type in the dashboard
    #[serde(rename = "color")]
    pub color: Color,
    /// When this type was created
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// Human readble description of this type
    #[serde(rename = "description")]
    pub description: String,
    /// If this is a dynamic catalog type, this will be the unique parameter for identitfying this resource externally.
    #[serde(
        rename = "dynamic_resource_parameter",
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_resource_parameter: Option<String>,
    /// If populated, gives an estimated count of entries for this type
    #[serde(rename = "estimated_count", skip_serializing_if = "Option::is_none")]
    pub estimated_count: Option<i64>,
    /// Sets the display icon of this type in the dashboard
    #[serde(rename = "icon")]
    pub icon: Icon,
    /// ID of this catalog type
    #[serde(rename = "id")]
    pub id: String,
    /// Catalog types that are synced with external resources can't be edited
    #[serde(rename = "is_editable")]
    pub is_editable: bool,
    /// When this type was last synced (if it's ever been sync'd)
    #[serde(rename = "last_synced_at", skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Name is the human readable name of this type
    #[serde(rename = "name")]
    pub name: String,
    /// If this type should be ranked
    #[serde(rename = "ranked")]
    pub ranked: bool,
    /// The registry resource this type is synced from, if any
    #[serde(rename = "registry_type", skip_serializing_if = "Option::is_none")]
    pub registry_type: Option<String>,
    /// If populated, the integrations required for this type
    #[serde(
        rename = "required_integrations",
        skip_serializing_if = "Option::is_none"
    )]
    pub required_integrations: Option<Vec<String>>,
    #[serde(rename = "schema")]
    pub schema: Box<models::CatalogTypeSchemaV2>,
    /// This type has been deprecated, and will always be empty.
    #[serde(rename = "semantic_type")]
    pub semantic_type: String,
    /// The url of the external repository where this type is managed
    #[serde(rename = "source_repo_url", skip_serializing_if = "Option::is_none")]
    pub source_repo_url: Option<String>,
    /// The type name of this catalog type, to be used when defining attributes. This is immutable once a CatalogType has been created. For non-externally sync types, it must follow the pattern Custom[\"SomeName\"]
    #[serde(rename = "type_name")]
    pub type_name: String,
    /// When this type was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl CatalogTypeV2 {
    pub fn new(
        annotations: std::collections::HashMap<String, String>,
        categories: Vec<Categories>,
        color: Color,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        description: String,
        icon: Icon,
        id: String,
        is_editable: bool,
        name: String,
        ranked: bool,
        schema: models::CatalogTypeSchemaV2,
        semantic_type: String,
        type_name: String,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> CatalogTypeV2 {
        CatalogTypeV2 {
            annotations,
            categories,
            color,
            created_at,
            description,
            dynamic_resource_parameter: None,
            estimated_count: None,
            icon,
            id,
            is_editable,
            last_synced_at: None,
            name,
            ranked,
            registry_type: None,
            required_integrations: None,
            schema: Box::new(schema),
            semantic_type,
            source_repo_url: None,
            type_name,
            updated_at,
        }
    }
}
/// What categories is this type considered part of
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Categories {
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "issue-tracker")]
    IssueTracker,
    #[serde(rename = "product-feature")]
    ProductFeature,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "on-call")]
    OnCall,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "user")]
    User,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for Categories {
    fn default() -> Categories {
        Self::Customer
    }
}
/// Sets the display color of this type in the dashboard
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Color {
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "violet")]
    Violet,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "orange")]
    Orange,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for Color {
    fn default() -> Color {
        Self::Yellow
    }
}
/// Sets the display icon of this type in the dashboard
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Icon {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "bolt")]
    Bolt,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "briefcase")]
    Briefcase,
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "bulb")]
    Bulb,
    #[serde(rename = "calendar")]
    Calendar,
    #[serde(rename = "clock")]
    Clock,
    #[serde(rename = "cog")]
    Cog,
    #[serde(rename = "components")]
    Components,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "doc")]
    Doc,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "escalation-path")]
    EscalationPath,
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "flag")]
    Flag,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "globe")]
    Globe,
    #[serde(rename = "incident-template")]
    IncidentTemplate,
    #[serde(rename = "money")]
    Money,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "severity")]
    Severity,
    #[serde(rename = "status-page")]
    StatusPage,
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "users")]
    Users,
    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
}

impl Default for Icon {
    fn default() -> Icon {
        Self::Alert
    }
}

// --- generated by scripts/fix_generated.py ---

impl CatalogTypeV2 {
    /// Sets `annotations`.
    pub fn set_annotations(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.annotations = value;
        self
    }

    /// Sets `categories`.
    pub fn set_categories(mut self, value: Vec<Categories>) -> Self {
        self.categories = value;
        self
    }

    /// Sets `color`.
    pub fn set_color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets `created_at`.
    pub fn set_created_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.created_at = value;
        self
    }

    /// Sets `description`.
    pub fn set_description(mut self, value: String) -> Self {
        self.description = value;
        self
    }

    /// Sets `dynamic_resource_parameter`.
    pub fn set_dynamic_resource_parameter(mut self, value: String) -> Self {
        self.dynamic_resource_parameter = Some(value);
        self
    }

    /// Sets `estimated_count`.
    pub fn set_estimated_count(mut self, value: i64) -> Self {
        self.estimated_count = Some(value);
        self
    }

    /// Sets `icon`.
    pub fn set_icon(mut self, value: Icon) -> Self {
        self.icon = value;
        self
    }

    /// Sets `id`.
    pub fn set_id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

    /// Sets `is_editable`.
    pub fn set_is_editable(mut self, value: bool) -> Self {
        self.is_editable = value;
        self
    }

    /// Sets `last_synced_at`.
    pub fn set_last_synced_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.last_synced_at = Some(value);
        self
    }

    /// Sets `name`.
    pub fn set_name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Sets `ranked`.
    pub fn set_ranked(mut self, value: bool) -> Self {
        self.ranked = value;
        self
    }

    /// Sets `registry_type`.
    pub fn set_registry_type(mut self, value: String) -> Self {
        self.registry_type = Some(value);
        self
    }

    /// Sets `required_integrations`.
    pub fn set_required_integrations(mut self, value: Vec<String>) -> Self {
        self.required_integrations = Some(value);
        self
    }

    /// Sets `schema`.
    pub fn set_schema(mut self, value: models::CatalogTypeSchemaV2) -> Self {
        self.schema = Box::new(value);
        self
    }

    /// Sets `semantic_type`.
    pub fn set_semantic_type(mut self, value: String) -> Self {
        self.semantic_type = value;
        self
    }

    /// Sets `source_repo_url`.
    pub fn set_source_repo_url(mut self, value: String) -> Self {
        self.source_repo_url = Some(value);
        self
    }

    /// Sets `type_name`.
    pub fn set_type_name(mut self, value: String) -> Self {
        self.type_name = value;
        self
    }

    /// Sets `updated_at`.
    pub fn set_updated_at(mut self, value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        self.updated_at = value;
        self
    }

}
