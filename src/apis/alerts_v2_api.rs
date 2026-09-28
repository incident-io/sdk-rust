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

/// struct for passing parameters to the method [`alerts_v2_add_tags`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2AddTagsParams {
    /// Unique identifier for the alert
    pub id: String,
    pub alerts_add_tags_payload_v2: models::AlertsAddTagsPayloadV2,
}

/// struct for passing parameters to the method [`alerts_v2_create_incident_alert`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2CreateIncidentAlertParams {
    pub alerts_create_incident_alert_payload_v2: models::AlertsCreateIncidentAlertPayloadV2,
}

/// struct for passing parameters to the method [`alerts_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2ListParams {
    /// Number of alerts to return per page
    pub page_size: i64,
    /// If provided, pass this as the 'after' param to load the next page
    pub after: Option<String>,
    /// Filter on alert deduplication key. The accepted operator is 'is'.
    pub deduplication_key: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on alert status. The accepted operators are 'one_of', or 'not_in'.
    pub status: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on alert source by ID. The accepted operators are 'one_of', or 'not_in'.
    pub alert_source: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on alert group ID. Returns alerts that belong to any of the specified groups. The accepted operator is 'one_of'.
    pub alert_group_id: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on alert created at timestamp. Accepted operators are 'gte', 'lte' and 'date_range'.
    pub created_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on alert updated at timestamp. Accepted operators are 'gte', 'lte' and 'date_range'.
    pub updated_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on an alerts attributes. Alert attribute ID should be sent, followed by the operator and values. Accepted operator will depend on the attribute type.
    pub attributes:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    /// Filter on whether an alert has notes. The accepted operator is 'is'.
    pub has_notes: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the tags applied to an alert, by tag name. The accepted operators are 'one_of', 'all_of' and 'not_in'.
    pub tags: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on whether to include maintenance window alerts. The accepted operator is 'is'.
    pub include_maintenance_window: Option<std::collections::HashMap<String, Vec<String>>>,
}

/// struct for passing parameters to the method [`alerts_v2_list_alert_tags`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2ListAlertTagsParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// A tag's ID. This endpoint will return a list of tags after this ID in relation to the API response order.
    pub after: Option<String>,
    /// Filter to tags whose name contains this value (case-insensitive)
    pub search: Option<String>,
}

/// struct for passing parameters to the method [`alerts_v2_list_incident_alerts`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2ListIncidentAlertsParams {
    /// Number of incident alerts to return per page
    pub page_size: i64,
    /// If provided, pass this as the 'after' param to load the next page
    pub after: Option<String>,
    /// Alert that this incident alert refers to
    pub alert_id: Option<String>,
    /// Incident that this incident alert is attached to
    pub incident_id: Option<String>,
}

/// struct for passing parameters to the method [`alerts_v2_remove_tags`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2RemoveTagsParams {
    /// Unique identifier for the alert
    pub id: String,
    pub alerts_remove_tags_payload_v2: models::AlertsRemoveTagsPayloadV2,
}

/// struct for passing parameters to the method [`alerts_v2_resolve`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2ResolveParams {
    /// Unique identifier for the alert
    pub id: String,
}

/// struct for passing parameters to the method [`alerts_v2_set_tags`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2SetTagsParams {
    /// Unique identifier for the alert
    pub id: String,
    pub alerts_set_tags_payload_v2: models::AlertsSetTagsPayloadV2,
}

/// struct for passing parameters to the method [`alerts_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2ShowParams {
    /// Unique identifier for the alert
    pub id: String,
}

/// struct for passing parameters to the method [`alerts_v2_transition_incident_alert`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AlertsV2TransitionIncidentAlertParams {
    /// Unique identifier for the incident alert
    pub id: String,
    pub alerts_transition_incident_alert_payload_v2: models::AlertsTransitionIncidentAlertPayloadV2,
}

/// struct for typed errors of method [`alerts_v2_add_tags`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2AddTagsError {
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

/// struct for typed errors of method [`alerts_v2_create_incident_alert`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2CreateIncidentAlertError {
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

/// struct for typed errors of method [`alerts_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2ListError {
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

/// struct for typed errors of method [`alerts_v2_list_alert_tags`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2ListAlertTagsError {
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

/// struct for typed errors of method [`alerts_v2_list_incident_alerts`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2ListIncidentAlertsError {
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

/// struct for typed errors of method [`alerts_v2_remove_tags`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2RemoveTagsError {
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

/// struct for typed errors of method [`alerts_v2_resolve`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2ResolveError {
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

/// struct for typed errors of method [`alerts_v2_set_tags`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2SetTagsError {
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

/// struct for typed errors of method [`alerts_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2ShowError {
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

/// struct for typed errors of method [`alerts_v2_transition_incident_alert`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertsV2TransitionIncidentAlertError {
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

/// Add tags to an alert without changing its existing tags.  Each tag is a name. A name that does not exist is created, except on private alerts, where only names already in the organisation's tag vocabulary can be used. Tags that are already present are ignored.
pub async fn alerts_v2_add_tags(
    configuration: &configuration::Configuration,
    params: AlertsV2AddTagsParams,
) -> Result<models::AlertsAddTagsResultV2, Error<AlertsV2AddTagsError>> {
    let uri_str = format!(
        "{}/v2/alerts/{id}/actions/add_tags",
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
    req_builder = req_builder.json(&params.alerts_add_tags_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsAddTagsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsAddTagsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2AddTagsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2AddTagsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Attach an alert to an incident, creating the connection between them.  The API key also needs the 'manage incident alerts' scope, which is what actually relates the alert once the connection exists.  If the alert is already related to this incident, the existing connection is returned unchanged. If someone previously marked the alert as unrelated to this incident, that decision is preserved and this endpoint returns a 422 — set re_relate to override it.  Private alerts can only be attached to private incidents, including when re_relate is set. An API key that cannot see the alert or the incident receives a 404.  Busy incidents can be locked by another operation, in which case this endpoint returns a 409 and the request can be retried.  Note that this endpoint returns 201 even when it re-relates or returns an existing connection rather than creating a new one.
pub async fn alerts_v2_create_incident_alert(
    configuration: &configuration::Configuration,
    params: AlertsV2CreateIncidentAlertParams,
) -> Result<models::AlertsCreateIncidentAlertResultV2, Error<AlertsV2CreateIncidentAlertError>> {
    let uri_str = format!("{}/v2/incident_alerts", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.alerts_create_incident_alert_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsCreateIncidentAlertResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsCreateIncidentAlertResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2CreateIncidentAlertError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2CreateIncidentAlertError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all alerts for your account.    This endpoint supports a number of filters, which can help find alerts matching certain criteria. These filters work similarly to the filters on the incidents endpoint, where  a field is specified alongside a comparison operator in the query string.  Note that: - Filters may be used together, and the result will be alerts that match all filters. - All query parameters must be URI encoded.  ### By deduplication_key  Find all alerts with deduplication_key ABC:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'deduplication_key[is]=ABC'  ### By status  Find all alerts in a firing state:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'status[one_of]=firing'  ### By alert_source  Find all alerts from a specific alert source (by alert source ID):    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'alert_source[one_of]=01GBSQF3FHF7FWZQNWGHAVQ804'  Find all alerts not from a specific alert source:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'alert_source[not_in]=01GBSQF3FHF7FWZQNWGHAVQ804'  ### By alert_group_id  Find all alerts in a specific alert group:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'alert_group_id[one_of]=01GBSQF3FHF7FWZQNWGHAVQ804'  ### By created_at Find all alerts that follow specified date parameters for created_at field. Possible values are \"gte\" (greater than or equal to), \"lte\" (less than or equal to), and  \"date_range\" (between two dates). The following example finds all alerts created after  2025-01-01:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'created_at[gte]=2025-01-01'  To find alerts created within a specific date range, use the date_range option with  tilde-separated dates:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'created_at[date_range]=2024-12-02~2024-12-08'  ### By updated_at Find all alerts that follow specified date parameters for updated_at field, using the same \"gte\", \"lte\" and \"date_range\" operators as created_at. This is useful for incrementally syncing alerts: poll with updated_at[gte] set to your last sync time instead of re-fetching the full history. Note that updated_at moves whenever the alert row is written, which can happen without a visible change to the alert payload, so treat matches as candidates to re-fetch rather than guaranteed changes — and overlap your sync window by a few minutes to allow for writes that commit out of timestamp order:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'updated_at[gte]=2025-01-01T00:00:00Z'  ### By has_notes  Find all alerts that have notes attached:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'has_notes[is]=true'  Find all alerts that have no notes attached:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'has_notes[is]=false'  ### By tags  Filter by tag name, matched case-insensitively.  Find all alerts that have any of the given tags:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'tags[one_of]=known issue'  Find all alerts that have all of the given tags:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'tags[all_of]=known issue' \\    --data 'tags[all_of]=customer impacting'  Find all alerts that do not have any of the given tags. Untagged alerts are included:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'tags[not_in]=known issue'  ### By attributes  Alerts can be filtered by their attribute values. Each filter is keyed by the alert attribute ID, followed by an operator and the values to match. The accepted operators depend on the attribute's type.  Find all alerts where attribute 01GBSQF3FHF7FWZQNWGHAVQ804 is one of two catalog entries:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'attributes[01GBSQF3FHF7FWZQNWGHAVQ804][one_of]=01GBSQF3FHF7FWZQNWGHAVQ804' \\    --data 'attributes[01GBSQF3FHF7FWZQNWGHAVQ804][one_of]=01ET65M7ZARSFZ6TFDFVQDN9AA'  You can filter on multiple attributes at once, and the result will be alerts that match all of them.  ### Maintenance windows By default, all alerts are returned including those held by a maintenance window. To exclude alerts that are held by a maintenance window:    curl --get 'https://api.incident.io/v2/alerts' \\    --data 'include_maintenance_window[is]=false'   
pub async fn alerts_v2_list(
    configuration: &configuration::Configuration,
    params: AlertsV2ListParams,
) -> Result<models::AlertsListResultV2, Error<AlertsV2ListError>> {
    let uri_str = format!("{}/v2/alerts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("page_size", &params.page_size.to_string())]);
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.deduplication_key {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "deduplication_key",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.status {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "status",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.alert_source {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "alert_source",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.alert_group_id {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "alert_group_id",
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
    if let Some(ref param_value) = params.attributes {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "attributes",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.has_notes {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "has_notes",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.tags {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "tags",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.include_maintenance_window {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "include_maintenance_window",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the alert tags in your organisation's vocabulary.  Alert tags are the reusable labels you can apply to alerts. This endpoint lists the organisation's active tags, ordered by name, and excludes archived tags.  To narrow the list, filter by search, which matches tags whose name contains the given value (case-insensitive):    curl --get 'https://api.incident.io/v2/alert_tags' \\    --data 'search=noisy'
pub async fn alerts_v2_list_alert_tags(
    configuration: &configuration::Configuration,
    params: AlertsV2ListAlertTagsParams,
) -> Result<models::AlertsListAlertTagsResultV2, Error<AlertsV2ListAlertTagsError>> {
    let uri_str = format!("{}/v2/alert_tags", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsListAlertTagsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsListAlertTagsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2ListAlertTagsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2ListAlertTagsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the connections between incidents and alerts
pub async fn alerts_v2_list_incident_alerts(
    configuration: &configuration::Configuration,
    params: AlertsV2ListIncidentAlertsParams,
) -> Result<models::AlertsListIncidentAlertsResultV2, Error<AlertsV2ListIncidentAlertsError>> {
    let uri_str = format!("{}/v2/incident_alerts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("page_size", &params.page_size.to_string())]);
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.alert_id {
        req_builder = req_builder.query(&[("alert_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.incident_id {
        req_builder = req_builder.query(&[("incident_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsListIncidentAlertsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsListIncidentAlertsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2ListIncidentAlertsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2ListIncidentAlertsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Remove tags from an alert without changing its other tags.  Each tag is a name. Tags that are not present on the alert are ignored.
pub async fn alerts_v2_remove_tags(
    configuration: &configuration::Configuration,
    params: AlertsV2RemoveTagsParams,
) -> Result<models::AlertsRemoveTagsResultV2, Error<AlertsV2RemoveTagsError>> {
    let uri_str = format!(
        "{}/v2/alerts/{id}/actions/remove_tags",
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
    req_builder = req_builder.json(&params.alerts_remove_tags_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsRemoveTagsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsRemoveTagsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2RemoveTagsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2RemoveTagsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Resolve a currently firing alert.  This marks the alert as resolved with the current time, attributing the resolution to the API key that made the request. Resolving an already-resolved alert is a no-op and returns the alert unchanged.  Some alert sources are 'externally resolved' (for example, Datadog) — those alerts can only be resolved by the third-party system itself, and this endpoint will return a 422 explaining that.  Private alerts: an API key without the 'view all alerts' scope can only resolve non-private alerts; private alerts will return a 404. Grant the API key a role that includes the global alerts access scope to resolve private alerts.
pub async fn alerts_v2_resolve(
    configuration: &configuration::Configuration,
    params: AlertsV2ResolveParams,
) -> Result<models::AlertsResolveResultV2, Error<AlertsV2ResolveError>> {
    let uri_str = format!(
        "{}/v2/alerts/{id}/actions/resolve",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsResolveResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsResolveResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2ResolveError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2ResolveError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Replace all tags on an alert.  Each tag is a name. A name that does not exist is created, except on private alerts, where only names already in the organisation's tag vocabulary can be used. Passing an empty list removes every tag.
pub async fn alerts_v2_set_tags(
    configuration: &configuration::Configuration,
    params: AlertsV2SetTagsParams,
) -> Result<models::AlertsSetTagsResultV2, Error<AlertsV2SetTagsError>> {
    let uri_str = format!(
        "{}/v2/alerts/{id}/actions/set_tags",
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
    req_builder = req_builder.json(&params.alerts_set_tags_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsSetTagsResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsSetTagsResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2SetTagsError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2SetTagsError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single alert for your account
pub async fn alerts_v2_show(
    configuration: &configuration::Configuration,
    params: AlertsV2ShowParams,
) -> Result<models::AlertsShowResultV2, Error<AlertsV2ShowError>> {
    let uri_str = format!(
        "{}/v2/alerts/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| AlertsV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Confirm or detach the connection between an alert and an incident.  Set state to 'related' to confirm the connection, or 'unrelated' to detach the alert from the incident. The API key also needs the 'manage incident alerts' scope.  A detached connection no longer appears in the list endpoint, which returns related connections only.  Busy incidents can be locked by another operation, in which case this endpoint returns a 409 and the request can be retried.
pub async fn alerts_v2_transition_incident_alert(
    configuration: &configuration::Configuration,
    params: AlertsV2TransitionIncidentAlertParams,
) -> Result<
    models::AlertsTransitionIncidentAlertResultV2,
    Error<AlertsV2TransitionIncidentAlertError>,
> {
    let uri_str = format!(
        "{}/v2/incident_alerts/{id}/actions/transition",
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
    req_builder = req_builder.json(&params.alerts_transition_incident_alert_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlertsTransitionIncidentAlertResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlertsTransitionIncidentAlertResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlertsV2TransitionIncidentAlertError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| AlertsV2TransitionIncidentAlertError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl AlertsV2AddTagsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        alerts_add_tags_payload_v2: models::AlertsAddTagsPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            alerts_add_tags_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `alerts_add_tags_payload_v2`.
    #[must_use]
    pub fn set_alerts_add_tags_payload_v2(mut self, value: models::AlertsAddTagsPayloadV2) -> Self {
        self.alerts_add_tags_payload_v2 = value;
        self
    }
}

impl AlertsV2CreateIncidentAlertParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        alerts_create_incident_alert_payload_v2: models::AlertsCreateIncidentAlertPayloadV2,
    ) -> Self {
        Self {
            alerts_create_incident_alert_payload_v2,
        }
    }

    /// Sets `alerts_create_incident_alert_payload_v2`.
    #[must_use]
    pub fn set_alerts_create_incident_alert_payload_v2(
        mut self,
        value: models::AlertsCreateIncidentAlertPayloadV2,
    ) -> Self {
        self.alerts_create_incident_alert_payload_v2 = value;
        self
    }
}

impl AlertsV2ListParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(page_size: i64) -> Self {
        Self {
            page_size,
            after: None,
            deduplication_key: None,
            status: None,
            alert_source: None,
            alert_group_id: None,
            created_at: None,
            updated_at: None,
            attributes: None,
            has_notes: None,
            tags: None,
            include_maintenance_window: None,
        }
    }

    /// Sets `page_size`.
    #[must_use]
    pub fn set_page_size(mut self, value: i64) -> Self {
        self.page_size = value;
        self
    }

    /// Sets `after`.
    #[must_use]
    pub fn set_after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Sets `deduplication_key`.
    #[must_use]
    pub fn set_deduplication_key(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.deduplication_key = Some(value);
        self
    }

    /// Sets `status`.
    #[must_use]
    pub fn set_status(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets `alert_source`.
    #[must_use]
    pub fn set_alert_source(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.alert_source = Some(value);
        self
    }

    /// Sets `alert_group_id`.
    #[must_use]
    pub fn set_alert_group_id(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.alert_group_id = Some(value);
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

    /// Sets `attributes`.
    #[must_use]
    pub fn set_attributes(
        mut self,
        value: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Sets `has_notes`.
    #[must_use]
    pub fn set_has_notes(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.has_notes = Some(value);
        self
    }

    /// Sets `tags`.
    #[must_use]
    pub fn set_tags(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Sets `include_maintenance_window`.
    #[must_use]
    pub fn set_include_maintenance_window(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.include_maintenance_window = Some(value);
        self
    }
}

impl AlertsV2ListAlertTagsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {
            page_size: None,
            after: None,
            search: None,
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

    /// Sets `search`.
    #[must_use]
    pub fn set_search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }
}

impl AlertsV2ListIncidentAlertsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(page_size: i64) -> Self {
        Self {
            page_size,
            after: None,
            alert_id: None,
            incident_id: None,
        }
    }

    /// Sets `page_size`.
    #[must_use]
    pub fn set_page_size(mut self, value: i64) -> Self {
        self.page_size = value;
        self
    }

    /// Sets `after`.
    #[must_use]
    pub fn set_after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Sets `alert_id`.
    #[must_use]
    pub fn set_alert_id(mut self, value: impl Into<String>) -> Self {
        self.alert_id = Some(value.into());
        self
    }

    /// Sets `incident_id`.
    #[must_use]
    pub fn set_incident_id(mut self, value: impl Into<String>) -> Self {
        self.incident_id = Some(value.into());
        self
    }
}

impl AlertsV2RemoveTagsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        alerts_remove_tags_payload_v2: models::AlertsRemoveTagsPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            alerts_remove_tags_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `alerts_remove_tags_payload_v2`.
    #[must_use]
    pub fn set_alerts_remove_tags_payload_v2(
        mut self,
        value: models::AlertsRemoveTagsPayloadV2,
    ) -> Self {
        self.alerts_remove_tags_payload_v2 = value;
        self
    }
}

impl AlertsV2ResolveParams {
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

impl AlertsV2SetTagsParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        alerts_set_tags_payload_v2: models::AlertsSetTagsPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            alerts_set_tags_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `alerts_set_tags_payload_v2`.
    #[must_use]
    pub fn set_alerts_set_tags_payload_v2(mut self, value: models::AlertsSetTagsPayloadV2) -> Self {
        self.alerts_set_tags_payload_v2 = value;
        self
    }
}

impl AlertsV2ShowParams {
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

impl AlertsV2TransitionIncidentAlertParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        alerts_transition_incident_alert_payload_v2: models::AlertsTransitionIncidentAlertPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            alerts_transition_incident_alert_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `alerts_transition_incident_alert_payload_v2`.
    #[must_use]
    pub fn set_alerts_transition_incident_alert_payload_v2(
        mut self,
        value: models::AlertsTransitionIncidentAlertPayloadV2,
    ) -> Self {
        self.alerts_transition_incident_alert_payload_v2 = value;
        self
    }
}

impl Default for AlertsV2ListAlertTagsParams {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertsV2AddTagsError {
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

impl AlertsV2CreateIncidentAlertError {
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

impl AlertsV2ListError {
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

impl AlertsV2ListAlertTagsError {
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

impl AlertsV2ListIncidentAlertsError {
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

impl AlertsV2RemoveTagsError {
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

impl AlertsV2ResolveError {
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

impl AlertsV2SetTagsError {
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

impl AlertsV2ShowError {
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

impl AlertsV2TransitionIncidentAlertError {
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
