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
pub struct EliminationAlliance {
    /// Alliance name, may be null.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "backup", skip_serializing_if = "Option::is_none")]
    pub backup: Option<Box<crate::models::EliminationAllianceBackup>>,
    /// List of teams that declined the alliance.
    #[serde(rename = "declines", skip_serializing_if = "Option::is_none")]
    pub declines: Option<Vec<String>>,
    /// List of team keys picked for the alliance. First pick is captain.
    #[serde(rename = "picks")]
    pub picks: Vec<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::EliminationAllianceStatus>>,
}

impl EliminationAlliance {
    pub fn new(picks: Vec<String>) -> EliminationAlliance {
        EliminationAlliance {
            name: None,
            backup: None,
            declines: None,
            picks,
            status: None,
        }
    }
}

