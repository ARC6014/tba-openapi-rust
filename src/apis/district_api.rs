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


/// struct for typed errors of method [`get_district_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictEventsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_events_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictEventsKeysError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_events_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictEventsSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_rankings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictRankingsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_teams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictTeamsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_teams_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictTeamsKeysError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_district_teams_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictTeamsSimpleError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_districts_by_year`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDistrictsByYearError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_event_district_points`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventDistrictPointsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_team_districts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamDistrictsError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// Gets a list of events in the given district.
pub async fn get_district_events(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::Event>, Option<String>)>, Error<GetDistrictEventsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/events", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictEventsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of event keys for events in the given district.
pub async fn get_district_events_keys(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<String>, Option<String>)>, Error<GetDistrictEventsKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/events/keys", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictEventsKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form list of events in the given district.
pub async fn get_district_events_simple(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::EventSimple>, Option<String>)>, Error<GetDistrictEventsSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/events/simple", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictEventsSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of team district rankings for the given district.
pub async fn get_district_rankings(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::DistrictRanking>, Option<String>)>, Error<GetDistrictRankingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/rankings", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictRankingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of `Team` objects that competed in events in the given district.
pub async fn get_district_teams(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::Team>, Option<String>)>, Error<GetDistrictTeamsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/teams", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictTeamsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of `Team` objects that competed in events in the given district.
pub async fn get_district_teams_keys(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<String>, Option<String>)>, Error<GetDistrictTeamsKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/teams/keys", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictTeamsKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a short-form list of `Team` objects that competed in events in the given district.
pub async fn get_district_teams_simple(configuration: &configuration::Configuration, district_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::TeamSimple>, Option<String>)>, Error<GetDistrictTeamsSimpleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/district/{district_key}/teams/simple", local_var_configuration.base_path, district_key=crate::apis::urlencode(district_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictTeamsSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of districts and their corresponding district key, for the given year.
pub async fn get_districts_by_year(configuration: &configuration::Configuration, year: i32, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::DistrictList>, Option<String>)>, Error<GetDistrictsByYearError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/districts/{year}", local_var_configuration.base_path, year=year);
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetDistrictsByYearError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of team rankings for the Event.
pub async fn get_event_district_points(configuration: &configuration::Configuration, event_key: &str, if_none_match: Option<&str>) -> Result<Option<(crate::models::EventDistrictPoints, Option<String>)>, Error<GetEventDistrictPointsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/event/{event_key}/district_points", local_var_configuration.base_path, event_key=crate::apis::urlencode(event_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetEventDistrictPointsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets an array of districts representing each year the team was in a district. Will return an empty array if the team was never in a district.
pub async fn get_team_districts(configuration: &configuration::Configuration, team_key: &str, if_none_match: Option<&str>) -> Result<Option<(Vec<crate::models::DistrictList>, Option<String>)>, Error<GetTeamDistrictsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team/{team_key}/districts", local_var_configuration.base_path, team_key=crate::apis::urlencode(team_key));
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
        if local_var_status.as_u16() == 304 {
            return Ok(None)
        }
        let local_var_entity = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(Some((local_var_entity, etag)))
    } else {
        let local_var_entity: Option<GetTeamDistrictsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

