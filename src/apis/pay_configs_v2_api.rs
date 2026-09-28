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

/// struct for passing parameters to the method [`pay_configs_v2_create`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2CreateParams {
    pub pay_configs_create_payload_v2: models::PayConfigsCreatePayloadV2,
}

/// struct for passing parameters to the method [`pay_configs_v2_create_one_off_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2CreateOneOffRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    pub pay_configs_create_one_off_rule_payload_v2: models::PayConfigsCreateOneOffRulePayloadV2,
}

/// struct for passing parameters to the method [`pay_configs_v2_create_weekly_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2CreateWeeklyRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    pub pay_configs_create_weekly_rule_payload_v2: models::PayConfigsCreateWeeklyRulePayloadV2,
}

/// struct for passing parameters to the method [`pay_configs_v2_destroy`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2DestroyParams {
    /// Unique identifier for this pay config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_destroy_one_off_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2DestroyOneOffRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_destroy_weekly_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2DestroyWeeklyRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ListParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// A pay config's ID. This endpoint will return a list of pay configs after this ID in relation to the API response order.
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`pay_configs_v2_list_one_off_rules`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ListOneOffRulesParams {
    /// The pay config's ID
    pub pay_config_id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_list_weekly_rules`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ListWeeklyRulesParams {
    /// The pay config's ID
    pub pay_config_id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ShowParams {
    /// Unique identifier for this pay config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_show_one_off_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ShowOneOffRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_show_weekly_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2ShowWeeklyRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
}

/// struct for passing parameters to the method [`pay_configs_v2_update`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2UpdateParams {
    /// Unique identifier for this pay config
    pub id: String,
    pub pay_configs_update_payload_v2: models::PayConfigsUpdatePayloadV2,
}

/// struct for passing parameters to the method [`pay_configs_v2_update_one_off_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2UpdateOneOffRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
    pub pay_configs_update_one_off_rule_payload_v2: models::PayConfigsUpdateOneOffRulePayloadV2,
}

/// struct for passing parameters to the method [`pay_configs_v2_update_weekly_rule`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PayConfigsV2UpdateWeeklyRuleParams {
    /// The pay config's ID
    pub pay_config_id: String,
    /// Unique identifier for this rule, stable across edits to the config
    pub id: String,
    pub pay_configs_update_weekly_rule_payload_v2: models::PayConfigsUpdateWeeklyRulePayloadV2,
}

/// struct for typed errors of method [`pay_configs_v2_create`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2CreateError {
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

/// struct for typed errors of method [`pay_configs_v2_create_one_off_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2CreateOneOffRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_create_weekly_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2CreateWeeklyRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_destroy`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2DestroyError {
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

/// struct for typed errors of method [`pay_configs_v2_destroy_one_off_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2DestroyOneOffRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_destroy_weekly_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2DestroyWeeklyRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ListError {
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

/// struct for typed errors of method [`pay_configs_v2_list_one_off_rules`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ListOneOffRulesError {
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

/// struct for typed errors of method [`pay_configs_v2_list_weekly_rules`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ListWeeklyRulesError {
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

/// struct for typed errors of method [`pay_configs_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ShowError {
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

/// struct for typed errors of method [`pay_configs_v2_show_one_off_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ShowOneOffRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_show_weekly_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2ShowWeeklyRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_update`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2UpdateError {
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

/// struct for typed errors of method [`pay_configs_v2_update_one_off_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2UpdateOneOffRuleError {
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

/// struct for typed errors of method [`pay_configs_v2_update_weekly_rule`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayConfigsV2UpdateWeeklyRuleError {
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

/// Create a pay config.  The config is created as a draft, and becomes visible to everyone in the organisation once a report that prices against it is published.
pub async fn pay_configs_v2_create(
    configuration: &configuration::Configuration,
    params: PayConfigsV2CreateParams,
) -> Result<models::PayConfigsCreateResultV2, Error<PayConfigsV2CreateError>> {
    let uri_str = format!("{}/v2/pay_configs", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.pay_configs_create_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsCreateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsCreateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2CreateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2CreateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Add a one-off rule to a pay config. It may not overlap a rule the config already has. Where it lands in the list has no effect on pricing.
pub async fn pay_configs_v2_create_one_off_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2CreateOneOffRuleParams,
) -> Result<models::PayConfigsCreateOneOffRuleResultV2, Error<PayConfigsV2CreateOneOffRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/one_off_rules",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id)
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
    req_builder = req_builder.json(&params.pay_configs_create_one_off_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsCreateOneOffRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsCreateOneOffRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2CreateOneOffRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2CreateOneOffRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Add a weekly rule to a pay config.  The rule is added last, so it is evaluated after every rule already on the config.
pub async fn pay_configs_v2_create_weekly_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2CreateWeeklyRuleParams,
) -> Result<models::PayConfigsCreateWeeklyRuleResultV2, Error<PayConfigsV2CreateWeeklyRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/weekly_rules",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id)
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
    req_builder = req_builder.json(&params.pay_configs_create_weekly_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsCreateWeeklyRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsCreateWeeklyRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2CreateWeeklyRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2CreateWeeklyRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a pay config.  Reports already published against this config keep their own copy of it, so their figures do not change. Any schedule using it as a default loses that default.
pub async fn pay_configs_v2_destroy(
    configuration: &configuration::Configuration,
    params: PayConfigsV2DestroyParams,
) -> Result<(), Error<PayConfigsV2DestroyError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{id}",
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
        let entity: Option<PayConfigsV2DestroyError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2DestroyError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Remove a one-off rule from a pay config. Time it covered falls back to the weekly rules.
pub async fn pay_configs_v2_destroy_one_off_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2DestroyOneOffRuleParams,
) -> Result<(), Error<PayConfigsV2DestroyOneOffRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/one_off_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
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
        let entity: Option<PayConfigsV2DestroyOneOffRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2DestroyOneOffRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Remove a weekly rule from a pay config. Time it covered falls back to the base rate, or to a later rule that also covers it.
pub async fn pay_configs_v2_destroy_weekly_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2DestroyWeeklyRuleParams,
) -> Result<(), Error<PayConfigsV2DestroyWeeklyRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/weekly_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
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
        let entity: Option<PayConfigsV2DestroyWeeklyRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2DestroyWeeklyRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List pay configs for this organisation.  Returns published configs, and drafts created through the API, which belong to nobody and so are visible to everyone. A draft a person created in the dashboard stays private to them. Archived configs are never returned.
pub async fn pay_configs_v2_list(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ListParams,
) -> Result<models::PayConfigsListResultV2, Error<PayConfigsV2ListError>> {
    let uri_str = format!("{}/v2/pay_configs", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List a pay config's one-off rules, in no particular order. They take precedence over weekly rules, and may not overlap, so at most one applies to any moment.
pub async fn pay_configs_v2_list_one_off_rules(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ListOneOffRulesParams,
) -> Result<models::PayConfigsListOneOffRulesResultV2, Error<PayConfigsV2ListOneOffRulesError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/one_off_rules",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsListOneOffRulesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsListOneOffRulesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ListOneOffRulesError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ListOneOffRulesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List a pay config's weekly rules, in evaluation order.  A shift is priced by the first rule that covers it, so order is meaningful. A created rule goes last, and there is no way to reorder rules once set: to put one earlier, replace the config through its own create endpoint.
pub async fn pay_configs_v2_list_weekly_rules(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ListWeeklyRulesParams,
) -> Result<models::PayConfigsListWeeklyRulesResultV2, Error<PayConfigsV2ListWeeklyRulesError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/weekly_rules",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsListWeeklyRulesResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsListWeeklyRulesResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ListWeeklyRulesError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ListWeeklyRulesError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single pay config.
pub async fn pay_configs_v2_show(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ShowParams,
) -> Result<models::PayConfigsShowResultV2, Error<PayConfigsV2ShowError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single one-off rule.
pub async fn pay_configs_v2_show_one_off_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ShowOneOffRuleParams,
) -> Result<models::PayConfigsShowOneOffRuleResultV2, Error<PayConfigsV2ShowOneOffRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/one_off_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsShowOneOffRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsShowOneOffRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ShowOneOffRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ShowOneOffRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Show a single weekly rule.
pub async fn pay_configs_v2_show_weekly_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2ShowWeeklyRuleParams,
) -> Result<models::PayConfigsShowWeeklyRuleResultV2, Error<PayConfigsV2ShowWeeklyRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/weekly_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsShowWeeklyRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsShowWeeklyRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2ShowWeeklyRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2ShowWeeklyRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a pay config's attributes.  This changes the config's name, timezone, currency and base rate. It does not touch the config's rules, which are set when the config is created.  Updating a config that a published report priced against additionally requires the schedule_pay_configs.update_published scope, because it changes the explanation of pay someone has already been sent.
pub async fn pay_configs_v2_update(
    configuration: &configuration::Configuration,
    params: PayConfigsV2UpdateParams,
) -> Result<models::PayConfigsUpdateResultV2, Error<PayConfigsV2UpdateError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{id}",
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
    req_builder = req_builder.json(&params.pay_configs_update_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsUpdateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsUpdateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2UpdateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2UpdateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a one-off rule. It may not be moved to overlap another rule on the same config.
pub async fn pay_configs_v2_update_one_off_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2UpdateOneOffRuleParams,
) -> Result<models::PayConfigsUpdateOneOffRuleResultV2, Error<PayConfigsV2UpdateOneOffRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/one_off_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.pay_configs_update_one_off_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsUpdateOneOffRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsUpdateOneOffRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2UpdateOneOffRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2UpdateOneOffRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update a weekly rule, leaving its position in the evaluation order alone.
pub async fn pay_configs_v2_update_weekly_rule(
    configuration: &configuration::Configuration,
    params: PayConfigsV2UpdateWeeklyRuleParams,
) -> Result<models::PayConfigsUpdateWeeklyRuleResultV2, Error<PayConfigsV2UpdateWeeklyRuleError>> {
    let uri_str = format!(
        "{}/v2/pay_configs/{pay_config_id}/weekly_rules/{id}",
        configuration.base_path,
        pay_config_id = crate::apis::urlencode(params.pay_config_id),
        id = crate::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.pay_configs_update_weekly_rule_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PayConfigsUpdateWeeklyRuleResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PayConfigsUpdateWeeklyRuleResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PayConfigsV2UpdateWeeklyRuleError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| PayConfigsV2UpdateWeeklyRuleError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl PayConfigsV2CreateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_configs_create_payload_v2: models::PayConfigsCreatePayloadV2) -> Self {
        Self {
            pay_configs_create_payload_v2,
        }
    }

    /// Sets `pay_configs_create_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_create_payload_v2(
        mut self,
        value: models::PayConfigsCreatePayloadV2,
    ) -> Self {
        self.pay_configs_create_payload_v2 = value;
        self
    }
}

impl PayConfigsV2CreateOneOffRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        pay_config_id: impl Into<String>,
        pay_configs_create_one_off_rule_payload_v2: models::PayConfigsCreateOneOffRulePayloadV2,
    ) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            pay_configs_create_one_off_rule_payload_v2,
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `pay_configs_create_one_off_rule_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_create_one_off_rule_payload_v2(
        mut self,
        value: models::PayConfigsCreateOneOffRulePayloadV2,
    ) -> Self {
        self.pay_configs_create_one_off_rule_payload_v2 = value;
        self
    }
}

impl PayConfigsV2CreateWeeklyRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        pay_config_id: impl Into<String>,
        pay_configs_create_weekly_rule_payload_v2: models::PayConfigsCreateWeeklyRulePayloadV2,
    ) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            pay_configs_create_weekly_rule_payload_v2,
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `pay_configs_create_weekly_rule_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_create_weekly_rule_payload_v2(
        mut self,
        value: models::PayConfigsCreateWeeklyRulePayloadV2,
    ) -> Self {
        self.pay_configs_create_weekly_rule_payload_v2 = value;
        self
    }
}

impl PayConfigsV2DestroyParams {
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

impl PayConfigsV2DestroyOneOffRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl PayConfigsV2DestroyWeeklyRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl PayConfigsV2ListParams {
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

impl PayConfigsV2ListOneOffRulesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }
}

impl PayConfigsV2ListWeeklyRulesParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }
}

impl PayConfigsV2ShowParams {
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

impl PayConfigsV2ShowOneOffRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl PayConfigsV2ShowWeeklyRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(pay_config_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

impl PayConfigsV2UpdateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        pay_configs_update_payload_v2: models::PayConfigsUpdatePayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            pay_configs_update_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `pay_configs_update_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_update_payload_v2(
        mut self,
        value: models::PayConfigsUpdatePayloadV2,
    ) -> Self {
        self.pay_configs_update_payload_v2 = value;
        self
    }
}

impl PayConfigsV2UpdateOneOffRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        pay_config_id: impl Into<String>,
        id: impl Into<String>,
        pay_configs_update_one_off_rule_payload_v2: models::PayConfigsUpdateOneOffRulePayloadV2,
    ) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
            pay_configs_update_one_off_rule_payload_v2,
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `pay_configs_update_one_off_rule_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_update_one_off_rule_payload_v2(
        mut self,
        value: models::PayConfigsUpdateOneOffRulePayloadV2,
    ) -> Self {
        self.pay_configs_update_one_off_rule_payload_v2 = value;
        self
    }
}

impl PayConfigsV2UpdateWeeklyRuleParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        pay_config_id: impl Into<String>,
        id: impl Into<String>,
        pay_configs_update_weekly_rule_payload_v2: models::PayConfigsUpdateWeeklyRulePayloadV2,
    ) -> Self {
        Self {
            pay_config_id: pay_config_id.into(),
            id: id.into(),
            pay_configs_update_weekly_rule_payload_v2,
        }
    }

    /// Sets `pay_config_id`.
    #[must_use]
    pub fn set_pay_config_id(mut self, value: impl Into<String>) -> Self {
        self.pay_config_id = value.into();
        self
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `pay_configs_update_weekly_rule_payload_v2`.
    #[must_use]
    pub fn set_pay_configs_update_weekly_rule_payload_v2(
        mut self,
        value: models::PayConfigsUpdateWeeklyRulePayloadV2,
    ) -> Self {
        self.pay_configs_update_weekly_rule_payload_v2 = value;
        self
    }
}

impl Default for PayConfigsV2ListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl PayConfigsV2CreateError {
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

impl PayConfigsV2CreateOneOffRuleError {
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

impl PayConfigsV2CreateWeeklyRuleError {
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

impl PayConfigsV2DestroyError {
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

impl PayConfigsV2DestroyOneOffRuleError {
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

impl PayConfigsV2DestroyWeeklyRuleError {
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

impl PayConfigsV2ListError {
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

impl PayConfigsV2ListOneOffRulesError {
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

impl PayConfigsV2ListWeeklyRulesError {
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

impl PayConfigsV2ShowError {
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

impl PayConfigsV2ShowOneOffRuleError {
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

impl PayConfigsV2ShowWeeklyRuleError {
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

impl PayConfigsV2UpdateError {
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

impl PayConfigsV2UpdateOneOffRuleError {
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

impl PayConfigsV2UpdateWeeklyRuleError {
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
