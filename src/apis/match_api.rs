/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_event_match_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventMatchTimeseriesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_event_matches`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventMatchesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_event_matches_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventMatchesKeysError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_event_matches_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventMatchesSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_match`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMatchError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_match_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMatchSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_match_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMatchTimeseriesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_match_zebra`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMatchZebraError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_event_matches`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamEventMatchesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_event_matches_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamEventMatchesKeysError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_event_matches_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamEventMatchesSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_matches_by_year`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamMatchesByYearError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_matches_by_year_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamMatchesByYearKeysError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_matches_by_year_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamMatchesByYearSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// Gets an array of Match Keys for the given event key that have timeseries data. Returns an empty array if no matches have timeseries data. *WARNING:* This is *not* official data, and is subject to a significant possibility of error, or missing data. Do not rely on this data for any purpose. In fact, pretend we made it up. *WARNING:* This endpoint and corresponding data models are under *active development* and may change at any time, including in breaking ways.
pub async fn get_event_match_timeseries(configuration: &configuration::Configuration, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<String>, Option<String>), Error<GetEventMatchTimeseriesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/event/{event_key}/matches/timeseries", local_var_configuration.base_path, event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetEventMatchTimeseriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of matches for the given event.
pub async fn get_event_matches(configuration: &configuration::Configuration, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<crate::models::Match>, Option<String>), Error<GetEventMatchesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/event/{event_key}/matches", local_var_configuration.base_path, event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetEventMatchesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of match keys for the given event.
pub async fn get_event_matches_keys(configuration: &configuration::Configuration, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<String>, Option<String>), Error<GetEventMatchesKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/event/{event_key}/matches/keys", local_var_configuration.base_path, event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetEventMatchesKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form list of matches for the given event.
pub async fn get_event_matches_simple(configuration: &configuration::Configuration, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<crate::models::MatchSimple>, Option<String>), Error<GetEventMatchesSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/event/{event_key}/matches/simple", local_var_configuration.base_path, event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetEventMatchesSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a `Match` object for the given match key.
pub async fn get_match(configuration: &configuration::Configuration, match_key: &str, if_none_match: Option<&str>) -> Result<(crate::models::Match, Option<String>), Error<GetMatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/match/{match_key}", local_var_configuration.base_path, match_key=crate::apis::urlencode(match_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetMatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form `Match` object for the given match key.
pub async fn get_match_simple(configuration: &configuration::Configuration, match_key: &str, if_none_match: Option<&str>) -> Result<(crate::models::MatchSimple, Option<String>), Error<GetMatchSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/match/{match_key}/simple", local_var_configuration.base_path, match_key=crate::apis::urlencode(match_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetMatchSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets an array of game-specific Match Timeseries objects for the given match key or an empty array if not available. *WARNING:* This is *not* official data, and is subject to a significant possibility of error, or missing data. Do not rely on this data for any purpose. In fact, pretend we made it up. *WARNING:* This endpoint and corresponding data models are under *active development* and may change at any time, including in breaking ways.
pub async fn get_match_timeseries(configuration: &configuration::Configuration, match_key: &str, if_none_match: Option<&str>) -> Result<(Vec<serde_json::Value>, Option<String>), Error<GetMatchTimeseriesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/match/{match_key}/timeseries", local_var_configuration.base_path, match_key=crate::apis::urlencode(match_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetMatchTimeseriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets Zebra MotionWorks data for a Match for the given match key.
pub async fn get_match_zebra(configuration: &configuration::Configuration, match_key: &str, if_none_match: Option<&str>) -> Result<(crate::models::Zebra, Option<String>), Error<GetMatchZebraError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/match/{match_key}/zebra_motionworks", local_var_configuration.base_path, match_key=crate::apis::urlencode(match_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetMatchZebraError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of matches for the given team and event.
pub async fn get_team_event_matches(configuration: &configuration::Configuration, team_key: &str, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<crate::models::Match>, Option<String>), Error<GetTeamEventMatchesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/event/{event_key}/matches", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamEventMatchesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of match keys for matches for the given team and event.
pub async fn get_team_event_matches_keys(configuration: &configuration::Configuration, team_key: &str, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<String>, Option<String>), Error<GetTeamEventMatchesKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/event/{event_key}/matches/keys", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamEventMatchesKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form list of matches for the given team and event.
pub async fn get_team_event_matches_simple(configuration: &configuration::Configuration, team_key: &str, event_key: &str, if_none_match: Option<&str>) -> Result<(Vec<crate::models::Match>, Option<String>), Error<GetTeamEventMatchesSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/event/{event_key}/matches/simple", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), event_key=crate::apis::urlencode(event_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamEventMatchesSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of matches for the given team and year.
pub async fn get_team_matches_by_year(configuration: &configuration::Configuration, team_key: &str, year: i32, if_none_match: Option<&str>) -> Result<(Vec<crate::models::Match>, Option<String>), Error<GetTeamMatchesByYearError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/matches/{year}", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), year=year);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamMatchesByYearError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of match keys for matches for the given team and year.
pub async fn get_team_matches_by_year_keys(configuration: &configuration::Configuration, team_key: &str, year: i32, if_none_match: Option<&str>) -> Result<(Vec<String>, Option<String>), Error<GetTeamMatchesByYearKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/matches/{year}/keys", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), year=year);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamMatchesByYearKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form list of matches for the given team and year.
pub async fn get_team_matches_by_year_simple(configuration: &configuration::Configuration, team_key: &str, year: i32, if_none_match: Option<&str>) -> Result<(Vec<crate::models::MatchSimple>, Option<String>), Error<GetTeamMatchesByYearSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/matches/{year}/simple", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key), year=year);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-TBA-Auth-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;
    
    let etag = local_var_resp.headers().get(reqwest::header::ETAG).map(|h| h.to_str());
    let etag = match etag {
        Some(e) if e.is_ok() => Some(e.unwrap().to_owned()),
        _ => None,
    };

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok((local_var_entity, etag))
    } else {
        let local_var_entity: Option<GetTeamMatchesByYearSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

