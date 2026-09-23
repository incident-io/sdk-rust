use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    SerdePathToError(serde_path_to_error::Error<serde_json::Error>),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::SerdePathToError(e) => (
                "serde",
                format!("{}: {}", e.path().to_string(), e.inner().to_string()),
            ),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::SerdePathToError(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<serde_path_to_error::Error<serde_json::Error>> for Error<T> {
    fn from(e: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::SerdePathToError(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod actions_v1_api;
pub mod actions_v2_api;
pub mod actions_v3_api;
pub mod alert_attributes_v2_api;
pub mod alert_events_v2_api;
pub mod alert_notes_v1_api;
pub mod alert_routes_v2_api;
pub mod alert_routes_v3_api;
pub mod alert_sources_v2_api;
pub mod alerts_v2_api;
pub mod api_keys_v1_api;
pub mod call_routes_v2_api;
pub mod call_sessions_v2_api;
pub mod call_transcript_entries_v2_api;
pub mod catalog_v2_api;
pub mod catalog_v3_api;
pub mod custom_field_options_v1_api;
pub mod custom_fields_v1_api;
pub mod custom_fields_v2_api;
pub mod escalation_path_templates_v2_api;
pub mod escalations_v2_api;
pub mod follow_ups_v2_api;
pub mod follow_ups_v3_api;
pub mod heartbeat_v2_api;
pub mod incident_activity_log_entries_v2_api;
pub mod incident_attachments_v1_api;
pub mod incident_memberships_v1_api;
pub mod incident_participant_workloads_v2_api;
pub mod incident_participants_v2_api;
pub mod incident_relationships_v1_api;
pub mod incident_roles_v1_api;
pub mod incident_roles_v2_api;
pub mod incident_statuses_v1_api;
pub mod incident_templates_v1_api;
pub mod incident_timeline_items_v2_api;
pub mod incident_timestamps_v2_api;
pub mod incident_types_v1_api;
pub mod incident_updates_v2_api;
pub mod incidents_v1_api;
pub mod incidents_v2_api;
pub mod ip_allowlists_v1_api;
pub mod maintenance_windows_v1_api;
pub mod pay_configs_v2_api;
pub mod pay_reports_v2_api;
pub mod policies_v2_api;
pub mod policy_findings_v2_api;
pub mod postmortem_documents_v1_api;
pub mod schedule_sync_targets_v2_api;
pub mod schedules_v2_api;
pub mod secrets_v2_api;
pub mod severities_v1_api;
pub mod status_pages_v1_api;
pub mod status_pages_v2_api;
pub mod teams_v3_api;
pub mod telemetry_v2_api;
pub mod users_v2_api;
pub mod utilities_v1_api;
pub mod workflow_runs_v2_api;
pub mod workflows_v2_api;

pub mod configuration;
