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

/// struct for passing parameters to the method [`incidents_v2_create`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IncidentsV2CreateParams {
    pub incidents_create_payload_v2: models::IncidentsCreatePayloadV2,
}

/// struct for passing parameters to the method [`incidents_v2_edit`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IncidentsV2EditParams {
    /// The unique identifier of the incident that you want to edit
    pub id: String,
    pub incidents_edit_payload_v2: models::IncidentsEditPayloadV2,
}

/// struct for passing parameters to the method [`incidents_v2_import_postmortem_document`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IncidentsV2ImportPostmortemDocumentParams {
    /// The unique identifier of the incident
    pub id: String,
    pub incidents_import_postmortem_document_payload_v2:
        models::IncidentsImportPostmortemDocumentPayloadV2,
}

/// struct for passing parameters to the method [`incidents_v2_list`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IncidentsV2ListParams {
    /// Integer number of records to return
    pub page_size: Option<i64>,
    /// An incident's ID. This endpoint will return a list of incidents after this ID in relation to the API response order.
    pub after: Option<String>,
    /// What order to return results in.
    pub sort_by: Option<String>,
    /// How to combine the filters: 'all' combines them with AND logic (all must match), 'any' combines them with OR logic (any can match). Defaults to 'all'.
    pub filter_mode: Option<String>,
    /// Filter on incident status. The accepted operators are 'one_of', or 'not_in'.
    pub status: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on the category of the incidents status. The accepted operators are 'one_of', or 'not_in'.
    pub status_category: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on incident created at timestamp. The accepted operators are 'gte', 'lte' and 'date_range'.
    pub created_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on incident updated at timestamp. The accepted operators are 'gte', 'lte' and 'date_range'.
    pub updated_at: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on incident severity. The accepted operators are 'one_of', 'not_in', 'gte', 'lte'.
    pub severity: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on incident type. The accepted operators are 'one_of, or 'not_in'.
    pub incident_type: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Filter on an incident role. Role ID should be sent, along with backlink attribute ID (if needed) followed by the operator and values. The accepted operators are 'one_of', 'is_blank'.
    pub incident_role:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    /// Filter on an incident custom field. Custom field ID should be sent, followed by the operator and values. Accepted operator will depend on the custom field type.
    pub custom_field:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    /// Filter on incident mode. The accepted operator is 'one_of'.  If this is not provided, this value defaults to `{\"one_of\": [\"standard\", \"retrospective\"] }`, meaning that test and tutorial incidents are not included.
    pub mode: Option<std::collections::HashMap<String, Vec<String>>>,
}

/// struct for passing parameters to the method [`incidents_v2_show`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IncidentsV2ShowParams {
    /// Unique identifier for the incident
    pub id: String,
}

/// struct for typed errors of method [`incidents_v2_create`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentsV2CreateError {
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

/// struct for typed errors of method [`incidents_v2_edit`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentsV2EditError {
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

/// struct for typed errors of method [`incidents_v2_import_postmortem_document`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentsV2ImportPostmortemDocumentError {
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

/// struct for typed errors of method [`incidents_v2_list`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentsV2ListError {
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

/// struct for typed errors of method [`incidents_v2_show`]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncidentsV2ShowError {
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

/// Create a new incident.  Note that if the incident mode is set to \"retrospective\" then the new incident will not be announced in Slack.
pub async fn incidents_v2_create(
    configuration: &configuration::Configuration,
    params: IncidentsV2CreateParams,
) -> Result<models::IncidentsCreateResultV2, Error<IncidentsV2CreateError>> {
    let uri_str = format!("{}/v2/incidents", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&params.incidents_create_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IncidentsCreateResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IncidentsCreateResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncidentsV2CreateError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| IncidentsV2CreateError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Edit an existing incident.  This endpoint allows you to edit the properties of an existing incident: e.g. set the severity or update custom fields.  When using this endpoint, only fields that are provided will be edited (omitted fields will be ignored).  The API key must have the scope corresponding to each property it changes:  - `incidents.update_name` for the name - `incidents.update_summary` for the summary - `incidents.update_severity` for the severity - `incidents.update_status` for the status - `incidents.update_custom_fields` for custom fields - `incidents.update_timestamps` for timestamps - `incidents.update_role_assignments` for role assignments - `incident_calls.create` to set the call URL - `incident_calls.destroy` when replacing an existing call URL
pub async fn incidents_v2_edit(
    configuration: &configuration::Configuration,
    params: IncidentsV2EditParams,
) -> Result<models::IncidentsEditResultV2, Error<IncidentsV2EditError>> {
    let uri_str = format!(
        "{}/v2/incidents/{id}/actions/edit",
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
    req_builder = req_builder.json(&params.incidents_edit_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IncidentsEditResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IncidentsEditResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncidentsV2EditError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| IncidentsV2EditError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Import a postmortem document from markdown into an incident.  The document content should be provided as GitHub-Flavored Markdown. It will be parsed and converted into the collaborative editor format, and a new postmortem document will be created for the incident.  If no main postmortem document exists for the incident, the imported document will become the main document.
pub async fn incidents_v2_import_postmortem_document(
    configuration: &configuration::Configuration,
    params: IncidentsV2ImportPostmortemDocumentParams,
) -> Result<
    models::IncidentsImportPostmortemDocumentResultV2,
    Error<IncidentsV2ImportPostmortemDocumentError>,
> {
    let uri_str = format!(
        "{}/v2/incidents/{id}/actions/import_postmortem_document",
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
    req_builder = req_builder.json(&params.incidents_import_postmortem_document_payload_v2);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IncidentsImportPostmortemDocumentResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IncidentsImportPostmortemDocumentResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncidentsV2ImportPostmortemDocumentError> = serde_json::from_str::<
            models::ErrorResponse,
        >(&content)
        .ok()
        .map(|body| IncidentsV2ImportPostmortemDocumentError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all incidents for an organisation.  This endpoint supports a number of filters, which can help find incidents matching certain criteria.  Filters are provided as query parameters, but due to the dynamic nature of what you can query by (different accounts have different custom fields, statuses, etc) they are more complex than most.  The maximum page size that can be requested is 250.  To help, here are some exemplar curl requests with a human description of what they search for.  Note that: - Filters may be combined using the filter_mode parameter: 'all' (default) requires all filters to match (AND logic), while 'any' requires at least one filter to match (OR logic). - IDs are normally in UUID format, but have been replaced with shorter strings to improve readability. - All query parameters must be URI encoded.  ### By status  With status of id=ABC, find all incidents that are set to that status:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'status[one_of]=ABC'  Or all incidents that are not set to status with id=ABC:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'status[not_in]=ABC'  ### By created_at or updated_at  Find all incidents that follow specified date parameters for created_at and updated_at fields. Possible values are \"gte\" (greater than or equal to), \"lte\" (less than or equal to), and \"date_range\" (between two dates). The following example finds all incidents created before or on 2021-01-02T00:00:00Z:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'created_at[lte]=2021-01-02'  To find incidents created within a specific date range, use the date_range option with tilde-separated dates:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'created_at[date_range]=2024-12-02~2024-12-08'  ### By status category  Find all incidents that are in a status category. Some categories use a different name in the API than the one shown in the dashboard — most notably \"live\" (shown as \"Active\") and \"learning\" (shown as \"Post-incident\"). The full mapping is:  | API value  | Shown in app as | | ---------- | --------------- | | triage     | Triage          | | live       | Active          | | learning   | Post-incident   | | paused     | Paused          | | closed     | Closed          | | declined   | Declined        | | canceled   | Canceled        | | merged     | Merged          |  For example, to find all incidents the dashboard shows as \"Active\", filter on the \"live\" category:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'status_category[one_of]=live'  Or all incidents that are not in a status category:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'status_category[not_in]=live'   ### By severity  With severity of id=ABC, find all incidents that are set to that severity:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'severity[one_of]=ABC'  Or all incidents where severity rank is greater-than-or-equal-to the rank of severity id=ABC:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'severity[gte]=ABC'  Or all incidents where severity rank is less-than-or-equal-to the rank of severity id=ABC:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'severity[lte]=ABC'  ### By incident type  With incident type of id=ABC, find all incidents that are of that type:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'incident_type[one_of]=ABC'  Or all incidents not of that type:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'incident_type[not_in]=ABC'  ### By incident mode  By default, we return standard and retrospective incidents. This means that test and tutorial incidents are filtered out. To override this behaviour, you can use the mode filter to specify which modes you want to get.  To find incidents of all modes:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'mode[one_of]=standard&mode[one_of]=retrospective&mode[one_of]=test&mode[one_of]=tutorial'  To find just test incidents:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'mode[one_of]=test'   ### By incident role  Roles and custom fields have another nested layer in the query parameter, to account for operations against any of the roles or custom fields created in the account.  With incident role id=ABC, find all incidents where that role is unset:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'incident_role[ABC][is_set]=true'  Or where the role has been set:    curl --get 'https://api.incident.io/v2/incidents' \\    --data 'incident_role[ABC][is_set]=false'  ### By option custom fields  With an option custom field id=ABC, all incidents that have field ABC set to the custom field option of id=XYZ:    curl \\    --get 'https://api.incident.io/v2/incidents' \\    --data 'custom_field[ABC][one_of]=XYZ'  Or all incidents that do not have custom field id=ABC set to option id=XYZ:    curl \\    --get 'https://api.incident.io/v2/incidents' \\    --data 'custom_field[ABC][not_in]=XYZ'  ### Sorting  By default, results are ordered by their creation date. You can use the sort_by parameter to reverse this order:    curl \\    --get 'https://api.incident.io/v2/incidents' \\    --data 'sort_by=created_at_oldest_first'
pub async fn incidents_v2_list(
    configuration: &configuration::Configuration,
    params: IncidentsV2ListParams,
) -> Result<models::IncidentsListResultV2, Error<IncidentsV2ListError>> {
    let uri_str = format!("{}/v2/incidents", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort_by {
        req_builder = req_builder.query(&[("sort_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filter_mode {
        req_builder = req_builder.query(&[("filter_mode", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.status {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "status",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.status_category {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "status_category",
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
    if let Some(ref param_value) = params.severity {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "severity",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.incident_type {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "incident_type",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.incident_role {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "incident_role",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.custom_field {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "custom_field",
            &serde_json::to_value(param_value)?,
        ));
    }
    if let Some(ref param_value) = params.mode {
        req_builder = req_builder.query(&crate::apis::parse_deep_object(
            "mode",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IncidentsListResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IncidentsListResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncidentsV2ListError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| IncidentsV2ListError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a single incident.  The ID supplied can be either the incident's full ID, or the numeric part of its reference. For example, to get INC-123, you could use either its full ID or:    curl \\    --get 'https://api.incident.io/v2/incidents/123
pub async fn incidents_v2_show(
    configuration: &configuration::Configuration,
    params: IncidentsV2ShowParams,
) -> Result<models::IncidentsShowResultV2, Error<IncidentsV2ShowError>> {
    let uri_str = format!(
        "{}/v2/incidents/{id}",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IncidentsShowResultV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IncidentsShowResultV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncidentsV2ShowError> =
            serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(|body| IncidentsV2ShowError::from_status(status.as_u16(), body));
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

// --- generated by scripts/fix_generated.py ---

impl IncidentsV2CreateParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(incidents_create_payload_v2: models::IncidentsCreatePayloadV2) -> Self {
        Self {
            incidents_create_payload_v2,
        }
    }

    /// Sets `incidents_create_payload_v2`.
    #[must_use]
    pub fn set_incidents_create_payload_v2(
        mut self,
        value: models::IncidentsCreatePayloadV2,
    ) -> Self {
        self.incidents_create_payload_v2 = value;
        self
    }
}

impl IncidentsV2EditParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        incidents_edit_payload_v2: models::IncidentsEditPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            incidents_edit_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `incidents_edit_payload_v2`.
    #[must_use]
    pub fn set_incidents_edit_payload_v2(mut self, value: models::IncidentsEditPayloadV2) -> Self {
        self.incidents_edit_payload_v2 = value;
        self
    }
}

impl IncidentsV2ImportPostmortemDocumentParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        incidents_import_postmortem_document_payload_v2: models::IncidentsImportPostmortemDocumentPayloadV2,
    ) -> Self {
        Self {
            id: id.into(),
            incidents_import_postmortem_document_payload_v2,
        }
    }

    /// Sets `id`.
    #[must_use]
    pub fn set_id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets `incidents_import_postmortem_document_payload_v2`.
    #[must_use]
    pub fn set_incidents_import_postmortem_document_payload_v2(
        mut self,
        value: models::IncidentsImportPostmortemDocumentPayloadV2,
    ) -> Self {
        self.incidents_import_postmortem_document_payload_v2 = value;
        self
    }
}

impl IncidentsV2ListParams {
    /// The required parameters. Set the optional ones with the
    /// `set_*` methods below.
    #[must_use]
    pub fn new() -> Self {
        Self {
            page_size: None,
            after: None,
            sort_by: None,
            filter_mode: None,
            status: None,
            status_category: None,
            created_at: None,
            updated_at: None,
            severity: None,
            incident_type: None,
            incident_role: None,
            custom_field: None,
            mode: None,
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

    /// Sets `sort_by`.
    #[must_use]
    pub fn set_sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Sets `filter_mode`.
    #[must_use]
    pub fn set_filter_mode(mut self, value: impl Into<String>) -> Self {
        self.filter_mode = Some(value.into());
        self
    }

    /// Sets `status`.
    #[must_use]
    pub fn set_status(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets `status_category`.
    #[must_use]
    pub fn set_status_category(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.status_category = Some(value);
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

    /// Sets `severity`.
    #[must_use]
    pub fn set_severity(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.severity = Some(value);
        self
    }

    /// Sets `incident_type`.
    #[must_use]
    pub fn set_incident_type(
        mut self,
        value: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        self.incident_type = Some(value);
        self
    }

    /// Sets `incident_role`.
    #[must_use]
    pub fn set_incident_role(
        mut self,
        value: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
    ) -> Self {
        self.incident_role = Some(value);
        self
    }

    /// Sets `custom_field`.
    #[must_use]
    pub fn set_custom_field(
        mut self,
        value: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
    ) -> Self {
        self.custom_field = Some(value);
        self
    }

    /// Sets `mode`.
    #[must_use]
    pub fn set_mode(mut self, value: std::collections::HashMap<String, Vec<String>>) -> Self {
        self.mode = Some(value);
        self
    }
}

impl IncidentsV2ShowParams {
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

impl Default for IncidentsV2ListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl IncidentsV2CreateError {
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

impl IncidentsV2EditError {
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

impl IncidentsV2ImportPostmortemDocumentError {
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

impl IncidentsV2ListError {
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

impl IncidentsV2ShowError {
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
