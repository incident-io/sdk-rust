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

/// struct for passing parameters to the method [`catalog_v3_bulk_update_entries`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3BulkUpdateEntriesParams {
    pub catalog_bulk_update_entries_payload_v3: models::CatalogBulkUpdateEntriesPayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_create_entry`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3CreateEntryParams {
    pub catalog_create_entry_payload_v3: models::CatalogCreateEntryPayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_create_type`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3CreateTypeParams {
    pub catalog_create_type_payload_v3: models::CatalogCreateTypePayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_destroy_entry`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3DestroyEntryParams {
    /// ID of this catalog entry
    pub id: String,
}

/// struct for passing parameters to the method [`catalog_v3_destroy_type`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3DestroyTypeParams {
    /// ID of this catalog type
    pub id: String,
}

/// struct for passing parameters to the method [`catalog_v3_list_entries`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3ListEntriesParams {
    /// ID of this catalog type
    pub catalog_type_id: String,
    /// The integer number of records to return
    pub page_size: i64,
    /// An record's ID. This endpoint will return a list of records after this ID in relation to the API response order.
    pub after: Option<String>,
    /// If specified, only entries with this identifier will be returned. This will search by ID, external ID, and aliases.  If 'use name as identifier' is enabled for the catalog type, this will also match on name.
    pub identifier: Option<String>,
}

/// struct for passing parameters to the method [`catalog_v3_show_entry`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3ShowEntryParams {
    /// ID of this catalog entry
    pub id: String,
    /// Whether to include details of all attribute links (forwards and backwards) within the response. Default behaviour (no query param) is to include only forward links. When expand is false, we only show attributes of the catalog entry itself.When expand is true, we show forward and backward links
    pub expand: Option<bool>,
}

/// struct for passing parameters to the method [`catalog_v3_show_type`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3ShowTypeParams {
    /// ID of this catalog type
    pub id: String,
}

/// struct for passing parameters to the method [`catalog_v3_update_entry`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3UpdateEntryParams {
    /// ID of this catalog entry
    pub id: String,
    pub catalog_update_entry_payload_v3: models::CatalogUpdateEntryPayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_update_type`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3UpdateTypeParams {
    /// ID of this catalog type
    pub id: String,
    pub catalog_update_type_payload_v3: models::CatalogUpdateTypePayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_update_type_schema`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3UpdateTypeSchemaParams {
    /// ID of this catalog type
    pub id: String,
    pub catalog_update_type_schema_payload_v3: models::CatalogUpdateTypeSchemaPayloadV3,
}

/// struct for passing parameters to the method [`catalog_v3_list_resources`]
///
/// This operation takes no parameters today. The struct exists so that
/// the first one the API adds is a field rather than a change to this
/// function's signature, which no attribute can absorb.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3ListResourcesParams {}

/// struct for passing parameters to the method [`catalog_v3_list_types`]
///
/// This operation takes no parameters today. The struct exists so that
/// the first one the API adds is a field rather than a change to this
/// function's signature, which no attribute can absorb.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CatalogV3ListTypesParams {}

/// struct for typed errors of method [`catalog_v3_bulk_update_entries`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3BulkUpdateEntriesError {
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

/// struct for typed errors of method [`catalog_v3_create_entry`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3CreateEntryError {
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

/// struct for typed errors of method [`catalog_v3_create_type`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3CreateTypeError {
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

/// struct for typed errors of method [`catalog_v3_destroy_entry`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3DestroyEntryError {
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

/// struct for typed errors of method [`catalog_v3_destroy_type`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3DestroyTypeError {
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

/// struct for typed errors of method [`catalog_v3_list_entries`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3ListEntriesError {
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

/// struct for typed errors of method [`catalog_v3_list_resources`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3ListResourcesError {
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

/// struct for typed errors of method [`catalog_v3_list_types`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3ListTypesError {
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

/// struct for typed errors of method [`catalog_v3_show_entry`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3ShowEntryError {
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

/// struct for typed errors of method [`catalog_v3_show_type`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3ShowTypeError {
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

/// struct for typed errors of method [`catalog_v3_update_entry`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3UpdateEntryError {
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

/// struct for typed errors of method [`catalog_v3_update_type`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3UpdateTypeError {
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

/// struct for typed errors of method [`catalog_v3_update_type_schema`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CatalogV3UpdateTypeSchemaError {
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

/// Update multiple catalog entries in a single operation. You can update up to 250 entries at once. This operation is atomic - either all entries are updated successfully, or none are updated.
pub async fn catalog_v3_bulk_update_entries(
    configuration: &configuration::Configuration,
    params: CatalogV3BulkUpdateEntriesParams,
) -> Result<(), Error<CatalogV3BulkUpdateEntriesError>> {
    let uri_str = format!(
        "{}/v3/catalog_entries/actions/bulk_update",
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
    req_builder = req_builder.json(&params.catalog_bulk_update_entries_payload_v3);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3BulkUpdateEntriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create an entry within the catalog. We support a maximum of 50,000 entries per type.  If you call this API with a payload where the external_id and catalog_type_id match an existing entry, the existing entry will be updated.
pub async fn catalog_v3_create_entry(
    configuration: &configuration::Configuration,
    params: CatalogV3CreateEntryParams,
) -> Result<models::CatalogCreateEntryResultV3, Error<CatalogV3CreateEntryError>> {
    let uri_str = format!("{}/v3/catalog_entries", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.catalog_create_entry_payload_v3);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogCreateEntryResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogCreateEntryResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3CreateEntryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a catalog type. The schema must be updated using the UpdateTypeSchema endpoint.
pub async fn catalog_v3_create_type(
    configuration: &configuration::Configuration,
    params: CatalogV3CreateTypeParams,
) -> Result<models::CatalogCreateTypeResultV3, Error<CatalogV3CreateTypeError>> {
    let uri_str = format!("{}/v3/catalog_types", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.catalog_create_type_payload_v3);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogCreateTypeResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogCreateTypeResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3CreateTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archives a catalog entry.
pub async fn catalog_v3_destroy_entry(
    configuration: &configuration::Configuration,
    params: CatalogV3DestroyEntryParams,
) -> Result<(), Error<CatalogV3DestroyEntryError>> {
    let uri_str = format!(
        "{}/v3/catalog_entries/{id}",
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
        let entity: Option<CatalogV3DestroyEntryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Archives a catalog type and associated entries.
pub async fn catalog_v3_destroy_type(
    configuration: &configuration::Configuration,
    params: CatalogV3DestroyTypeParams,
) -> Result<(), Error<CatalogV3DestroyTypeError>> {
    let uri_str = format!(
        "{}/v3/catalog_types/{id}",
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
        let entity: Option<CatalogV3DestroyTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List entries for a catalog type.
pub async fn catalog_v3_list_entries(
    configuration: &configuration::Configuration,
    params: CatalogV3ListEntriesParams,
) -> Result<models::CatalogListEntriesResultV3, Error<CatalogV3ListEntriesError>> {
    let uri_str = format!("{}/v3/catalog_entries", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("catalog_type_id", &params.catalog_type_id.to_string())]);
    req_builder = req_builder.query(&[("page_size", &params.page_size.to_string())]);
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.identifier {
        req_builder = req_builder.query(&[("identifier", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogListEntriesResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogListEntriesResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3ListEntriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List available engine resources for the catalog.  A resource represents a type of data that can be held within the catalog, so this endpoint can be used to see what attribute types can be used when updating the schema of a catalog type.
pub async fn catalog_v3_list_resources(
    configuration: &configuration::Configuration,
    params: CatalogV3ListResourcesParams,
) -> Result<models::CatalogListResourcesResultV3, Error<CatalogV3ListResourcesError>> {
    let _ = params;

    let uri_str = format!("{}/v3/catalog_resources", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogListResourcesResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogListResourcesResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3ListResourcesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all catalog types for an organisation, including those synced from external resources.
pub async fn catalog_v3_list_types(
    configuration: &configuration::Configuration,
    params: CatalogV3ListTypesParams,
) -> Result<models::CatalogListTypesResultV3, Error<CatalogV3ListTypesError>> {
    let _ = params;

    let uri_str = format!("{}/v3/catalog_types", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogListTypesResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogListTypesResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3ListTypesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single catalog entry.
pub async fn catalog_v3_show_entry(
    configuration: &configuration::Configuration,
    params: CatalogV3ShowEntryParams,
) -> Result<models::CatalogShowEntryResultV3, Error<CatalogV3ShowEntryError>> {
    let uri_str = format!(
        "{}/v3/catalog_entries/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogShowEntryResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogShowEntryResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3ShowEntryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single catalog type.
pub async fn catalog_v3_show_type(
    configuration: &configuration::Configuration,
    params: CatalogV3ShowTypeParams,
) -> Result<models::CatalogShowTypeResultV3, Error<CatalogV3ShowTypeError>> {
    let uri_str = format!(
        "{}/v3/catalog_types/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogShowTypeResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogShowTypeResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3ShowTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Updates an existing catalog entry.
pub async fn catalog_v3_update_entry(
    configuration: &configuration::Configuration,
    params: CatalogV3UpdateEntryParams,
) -> Result<models::CatalogUpdateEntryResultV3, Error<CatalogV3UpdateEntryError>> {
    let uri_str = format!(
        "{}/v3/catalog_entries/{id}",
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
    req_builder = req_builder.json(&params.catalog_update_entry_payload_v3);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogUpdateEntryResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogUpdateEntryResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3UpdateEntryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Updates an existing catalog type. The schema must be updated using the UpdateTypeSchema endpoint.
pub async fn catalog_v3_update_type(
    configuration: &configuration::Configuration,
    params: CatalogV3UpdateTypeParams,
) -> Result<models::CatalogUpdateTypeResultV3, Error<CatalogV3UpdateTypeError>> {
    let uri_str = format!(
        "{}/v3/catalog_types/{id}",
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
    req_builder = req_builder.json(&params.catalog_update_type_payload_v3);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogUpdateTypeResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogUpdateTypeResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3UpdateTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update an existing catalog types schema, adding or removing attributes.  Updating the schema is handled separately from creating and updating types, so that you don't have to worry about dependencies between types. For example, if type A has an attribute that relies on type B, you would have to create type B first.  By allowing the creation of types without a schema, they can be created in any order, but it means that you need to make a separate call to this endpoint to update the schema.
pub async fn catalog_v3_update_type_schema(
    configuration: &configuration::Configuration,
    params: CatalogV3UpdateTypeSchemaParams,
) -> Result<models::CatalogUpdateTypeSchemaResultV3, Error<CatalogV3UpdateTypeSchemaError>> {
    let uri_str = format!(
        "{}/v3/catalog_types/{id}/actions/update_schema",
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
    req_builder = req_builder.json(&params.catalog_update_type_schema_payload_v3);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CatalogUpdateTypeSchemaResultV3`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CatalogUpdateTypeSchemaResultV3`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CatalogV3UpdateTypeSchemaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl CatalogV3BulkUpdateEntriesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        catalog_bulk_update_entries_payload_v3: models::CatalogBulkUpdateEntriesPayloadV3,
    ) -> Self {
        Self {
            catalog_bulk_update_entries_payload_v3,
        }
    }

    /// Sets `catalog_bulk_update_entries_payload_v3`.
    #[must_use]
    pub fn set_catalog_bulk_update_entries_payload_v3(
        mut self,
        value: models::CatalogBulkUpdateEntriesPayloadV3,
    ) -> Self {
        self.catalog_bulk_update_entries_payload_v3 = value;
        self
    }
}

impl CatalogV3CreateEntryParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(catalog_create_entry_payload_v3: models::CatalogCreateEntryPayloadV3) -> Self {
        Self {
            catalog_create_entry_payload_v3,
        }
    }

    /// Sets `catalog_create_entry_payload_v3`.
    #[must_use]
    pub fn set_catalog_create_entry_payload_v3(
        mut self,
        value: models::CatalogCreateEntryPayloadV3,
    ) -> Self {
        self.catalog_create_entry_payload_v3 = value;
        self
    }
}

impl CatalogV3CreateTypeParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(catalog_create_type_payload_v3: models::CatalogCreateTypePayloadV3) -> Self {
        Self {
            catalog_create_type_payload_v3,
        }
    }

    /// Sets `catalog_create_type_payload_v3`.
    #[must_use]
    pub fn set_catalog_create_type_payload_v3(
        mut self,
        value: models::CatalogCreateTypePayloadV3,
    ) -> Self {
        self.catalog_create_type_payload_v3 = value;
        self
    }
}

impl CatalogV3DestroyEntryParams {
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

impl CatalogV3DestroyTypeParams {
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

impl CatalogV3ListEntriesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(catalog_type_id: impl Into<String>, page_size: i64) -> Self {
        Self {
            catalog_type_id: catalog_type_id.into(),
            page_size,
            after: None,
            identifier: None,
        }
    }

    /// Sets `catalog_type_id`.
    #[must_use]
    pub fn set_catalog_type_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_type_id = value.into();
        self
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

    /// Sets `identifier`.
    #[must_use]
    pub fn set_identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }
}

impl CatalogV3ShowEntryParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            expand: None,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `expand`.
    #[must_use]
    pub fn set_expand(mut self, value: bool) -> Self {
        self.expand = Some(value);
        self
    }
}

impl CatalogV3ShowTypeParams {
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

impl CatalogV3UpdateEntryParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        catalog_update_entry_payload_v3: models::CatalogUpdateEntryPayloadV3,
    ) -> Self {
        Self {
            id: id.into(),
            catalog_update_entry_payload_v3,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `catalog_update_entry_payload_v3`.
    #[must_use]
    pub fn set_catalog_update_entry_payload_v3(
        mut self,
        value: models::CatalogUpdateEntryPayloadV3,
    ) -> Self {
        self.catalog_update_entry_payload_v3 = value;
        self
    }
}

impl CatalogV3UpdateTypeParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        catalog_update_type_payload_v3: models::CatalogUpdateTypePayloadV3,
    ) -> Self {
        Self {
            id: id.into(),
            catalog_update_type_payload_v3,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `catalog_update_type_payload_v3`.
    #[must_use]
    pub fn set_catalog_update_type_payload_v3(
        mut self,
        value: models::CatalogUpdateTypePayloadV3,
    ) -> Self {
        self.catalog_update_type_payload_v3 = value;
        self
    }
}

impl CatalogV3UpdateTypeSchemaParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        catalog_update_type_schema_payload_v3: models::CatalogUpdateTypeSchemaPayloadV3,
    ) -> Self {
        Self {
            id: id.into(),
            catalog_update_type_schema_payload_v3,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `catalog_update_type_schema_payload_v3`.
    #[must_use]
    pub fn set_catalog_update_type_schema_payload_v3(
        mut self,
        value: models::CatalogUpdateTypeSchemaPayloadV3,
    ) -> Self {
        self.catalog_update_type_schema_payload_v3 = value;
        self
    }
}

impl CatalogV3ListResourcesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl CatalogV3ListTypesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CatalogV3ListResourcesParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CatalogV3ListTypesParams {
    fn default() -> Self {
        Self::new()
    }
}
