/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Team {
    /// TBA team key with the format `frcXXXX` with `XXXX` representing the team number.
    #[serde(rename = "key")]
    pub key: String,
    /// Official team number issued by FIRST.
    #[serde(rename = "team_number")]
    pub team_number: i32,
    /// Team nickname provided by FIRST.
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Official long name registered with FIRST.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of team school or affilited group registered with FIRST.
    #[serde(rename = "school_name", skip_serializing_if = "Option::is_none")]
    pub school_name: Option<String>,
    /// City of team derived from parsing the address registered with FIRST.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State of team derived from parsing the address registered with FIRST.
    #[serde(rename = "state_prov", skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    /// Country of team derived from parsing the address registered with FIRST.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Will be NULL, for future development.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal code from the team address.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Will be NULL, for future development.
    #[serde(rename = "gmaps_place_id", skip_serializing_if = "Option::is_none")]
    pub gmaps_place_id: Option<String>,
    /// Will be NULL, for future development.
    #[serde(rename = "gmaps_url", skip_serializing_if = "Option::is_none")]
    pub gmaps_url: Option<String>,
    /// Will be NULL, for future development.
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// Will be NULL, for future development.
    #[serde(rename = "lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    /// Will be NULL, for future development.
    #[serde(rename = "location_name", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Official website associated with the team.
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// First year the team officially competed.
    #[serde(rename = "rookie_year", skip_serializing_if = "Option::is_none")]
    pub rookie_year: Option<i32>,
    /// Team's motto as provided by FIRST. This field is deprecated and will return null - will be removed at end-of-season in 2019.
    #[serde(rename = "motto", skip_serializing_if = "Option::is_none")]
    pub motto: Option<String>,
    /// Location of the team's home championship each year as a key-value pair. The year (as a string) is the key, and the city is the value.
    #[serde(rename = "home_championship", skip_serializing_if = "Option::is_none")]
    pub home_championship: Option<serde_json::Value>,
}

impl Team {
    pub fn new(key: String, team_number: i32, name: String) -> Team {
        Team {
            key,
            team_number,
            nickname: None,
            name,
            school_name: None,
            city: None,
            state_prov: None,
            country: None,
            address: None,
            postal_code: None,
            gmaps_place_id: None,
            gmaps_url: None,
            lat: None,
            lng: None,
            location_name: None,
            website: None,
            rookie_year: None,
            motto: None,
            home_championship: None,
        }
    }
}


