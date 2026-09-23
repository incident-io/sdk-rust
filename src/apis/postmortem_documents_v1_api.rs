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

/// struct for passing parameters to the method [`postmortem_documents_v1_attach`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PostmortemDocumentsV1AttachParams {
    pub postmortem_documents_attach_payload_v1: models::PostmortemDocumentsAttachPayloadV1,
}

/// struct for passing parameters to the method [`postmortem_documents_v1_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PostmortemDocumentsV1ListParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// A post-mortem document's ID. This endpoint will return a list of post-mortem documents after this ID.
    pub after: Option<String>,
    /// Filter to only return post-mortem documents for the given incident
    pub incident_id: Option<String>,
    /// Controls the order that results are returned in
    pub sort_by: Option<String>,
}

/// struct for passing parameters to the method [`postmortem_documents_v1_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PostmortemDocumentsV1ShowParams {
    /// Unique identifier for the post-mortem document
    pub id: String,
}

/// struct for passing parameters to the method [`postmortem_documents_v1_show_content`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PostmortemDocumentsV1ShowContentParams {
    /// Unique identifier for the post-mortem document
    pub id: String,
}

/// struct for passing parameters to the method [`postmortem_documents_v1_update_status`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PostmortemDocumentsV1UpdateStatusParams {
    /// Unique identifier for the post-mortem document
    pub id: String,
    pub postmortem_documents_update_status_payload_v1:
        models::PostmortemDocumentsUpdateStatusPayloadV1,
}

/// struct for typed errors of method [`postmortem_documents_v1_attach`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostmortemDocumentsV1AttachError {
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

/// struct for typed errors of method [`postmortem_documents_v1_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostmortemDocumentsV1ListError {
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

/// struct for typed errors of method [`postmortem_documents_v1_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostmortemDocumentsV1ShowError {
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

/// struct for typed errors of method [`postmortem_documents_v1_show_content`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostmortemDocumentsV1ShowContentError {
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

/// struct for typed errors of method [`postmortem_documents_v1_update_status`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostmortemDocumentsV1UpdateStatusError {
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

/// Link an externally-hosted post-mortem document to an incident.  Use this to attach a retrospective document you've created in your own provider (for example Confluence, Notion, or Google Docs) to an existing incident. This is the API equivalent of pasting a document link into the incident.io dashboard, and is useful for automating your retrospective workflow - for example, creating a document in the right space with the right permissions when an incident is opened, then linking it back to the incident.  Only one external post-mortem can be attached to an incident, and you cannot attach an external document to an incident that already has an in-app post-mortem. Re-attaching the same incident with a new permalink updates the existing link.
pub async fn postmortem_documents_v1_attach(
    configuration: &configuration::Configuration,
    params: PostmortemDocumentsV1AttachParams,
) -> Result<models::PostmortemDocumentsAttachResultV1, Error<PostmortemDocumentsV1AttachError>> {
    let uri_str = format!(
        "{}/v1/postmortem_documents/actions/attach",
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
    req_builder = req_builder.json(&params.postmortem_documents_attach_payload_v1);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostmortemDocumentsAttachResultV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostmortemDocumentsAttachResultV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostmortemDocumentsV1AttachError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List post-mortem documents for the organisation.  Results can be filtered by incident and sorted by creation date. This endpoint returns document metadata only. If you want to fetch the content of the post-mortem, use the ShowContent endpoint.
pub async fn postmortem_documents_v1_list(
    configuration: &configuration::Configuration,
    params: PostmortemDocumentsV1ListParams,
) -> Result<models::PostmortemDocumentsListResultV1, Error<PostmortemDocumentsV1ListError>> {
    let uri_str = format!("{}/v1/postmortem_documents", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.incident_id {
        req_builder = req_builder.query(&[("incident_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort_by {
        req_builder = req_builder.query(&[("sort_by", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostmortemDocumentsListResultV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostmortemDocumentsListResultV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostmortemDocumentsV1ListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single post-mortem document by ID.  This returns the document's metadata. To retrieve the content of the post-mortem, use the ShowContent endpoint.
pub async fn postmortem_documents_v1_show(
    configuration: &configuration::Configuration,
    params: PostmortemDocumentsV1ShowParams,
) -> Result<models::PostmortemDocumentsShowResultV1, Error<PostmortemDocumentsV1ShowError>> {
    let uri_str = format!(
        "{}/v1/postmortem_documents/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostmortemDocumentsShowResultV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostmortemDocumentsShowResultV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostmortemDocumentsV1ShowError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetch the content of a post-mortem document, rendered as markdown.  The response contains the full document content as a single markdown string. The markdown follows standard formatting and is structured to mirror the post-mortem as it appears in the incident.io dashboard:  - **Headings** (`#`, `##`, `###`) for document title and sections - **Bold** and *italic* text formatting - Bullet lists and numbered lists - [Links](url) to external resources, Slack threads, and pull requests - Mentions of users, incidents, and catalog entries resolved to their display names - Custom field values rendered as labelled bullet lists - Timeline entries grouped by date with timestamps - Follow-ups with assignees and descriptions  To preview what this markdown will look like for a given post-mortem, open the document in the incident.io dashboard and use the \"Copy to clipboard\" button. The copied content uses the same rendering pipeline as this endpoint.  If you only need document metadata, use the Show or List endpoints instead.
pub async fn postmortem_documents_v1_show_content(
    configuration: &configuration::Configuration,
    params: PostmortemDocumentsV1ShowContentParams,
) -> Result<
    models::PostmortemDocumentsShowContentResultV1,
    Error<PostmortemDocumentsV1ShowContentError>,
> {
    let uri_str = format!(
        "{}/v1/postmortem_documents/{id}/content",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostmortemDocumentsShowContentResultV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostmortemDocumentsShowContentResultV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostmortemDocumentsV1ShowContentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the status of a post-mortem document.
pub async fn postmortem_documents_v1_update_status(
    configuration: &configuration::Configuration,
    params: PostmortemDocumentsV1UpdateStatusParams,
) -> Result<
    models::PostmortemDocumentsUpdateStatusResultV1,
    Error<PostmortemDocumentsV1UpdateStatusError>,
> {
    let uri_str = format!(
        "{}/v1/postmortem_documents/{id}",
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
    req_builder = req_builder.json(&params.postmortem_documents_update_status_payload_v1);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostmortemDocumentsUpdateStatusResultV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostmortemDocumentsUpdateStatusResultV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostmortemDocumentsV1UpdateStatusError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl PostmortemDocumentsV1AttachParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    pub fn new(postmortem_documents_attach_payload_v1: models::PostmortemDocumentsAttachPayloadV1) -> Self {
        Self {
            postmortem_documents_attach_payload_v1,
        }
    }

    /// Sets `postmortem_documents_attach_payload_v1`.
    pub fn set_postmortem_documents_attach_payload_v1(mut self, value: models::PostmortemDocumentsAttachPayloadV1) -> Self {
        self.postmortem_documents_attach_payload_v1 = value;
        self
    }

}

impl PostmortemDocumentsV1ListParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    pub fn new() -> Self {
        Self {
            page_size: None,
            after: None,
            incident_id: None,
            sort_by: None,
        }
    }

    /// Sets `page_size`.
    pub fn set_page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Sets `after`.
    pub fn set_after(mut self, value: String) -> Self {
        self.after = Some(value);
        self
    }

    /// Sets `incident_id`.
    pub fn set_incident_id(mut self, value: String) -> Self {
        self.incident_id = Some(value);
        self
    }

    /// Sets `sort_by`.
    pub fn set_sort_by(mut self, value: String) -> Self {
        self.sort_by = Some(value);
        self
    }

}

impl PostmortemDocumentsV1ShowParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    pub fn new(id: String) -> Self {
        Self {
            id,
        }
    }

    /// Sets `id`.
    pub fn set_id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

}

impl PostmortemDocumentsV1ShowContentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    pub fn new(id: String) -> Self {
        Self {
            id,
        }
    }

    /// Sets `id`.
    pub fn set_id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

}

impl PostmortemDocumentsV1UpdateStatusParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    pub fn new(id: String, postmortem_documents_update_status_payload_v1: models::PostmortemDocumentsUpdateStatusPayloadV1) -> Self {
        Self {
            id,
            postmortem_documents_update_status_payload_v1,
        }
    }

    /// Sets `id`.
    pub fn set_id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

    /// Sets `postmortem_documents_update_status_payload_v1`.
    pub fn set_postmortem_documents_update_status_payload_v1(mut self, value: models::PostmortemDocumentsUpdateStatusPayloadV1) -> Self {
        self.postmortem_documents_update_status_payload_v1 = value;
        self
    }

}

impl Default for PostmortemDocumentsV1ListParams {
    fn default() -> Self {
        Self::new()
    }
}
