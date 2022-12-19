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
pub struct ZebraTeam {
    /// The TBA team key for the Zebra MotionWorks data.
    #[serde(rename = "team_key")]
    pub team_key: String,
    /// A list containing doubles and nulls representing a teams X position in feet at the corresponding timestamp. A null value represents no tracking data for a given timestamp.
    #[serde(rename = "xs")]
    pub xs: Vec<f64>,
    /// A list containing doubles and nulls representing a teams Y position in feet at the corresponding timestamp. A null value represents no tracking data for a given timestamp.
    #[serde(rename = "ys")]
    pub ys: Vec<f64>,
}

impl ZebraTeam {
    pub fn new(team_key: String, xs: Vec<f64>, ys: Vec<f64>) -> ZebraTeam {
        ZebraTeam {
            team_key,
            xs,
            ys,
        }
    }
}

