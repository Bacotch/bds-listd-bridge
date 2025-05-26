use serde::{Deserialize, Serialize};
use serde_json;
pub type PlayerId = i64;

#[derive(Deserialize, Clone)]
pub struct ListdAction {
    #[serde(rename = "isStats")]
    pub is_stats: bool,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
}

#[derive(Deserialize)]
pub struct ListdInfo {
    pub command: String,
    pub result: Vec<ListdResult>,
}
#[derive(Serialize, Deserialize)]
pub struct ListdResult {
    #[serde(rename = "activeSessionId")]
    pub active_session_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub color: String,
    #[serde(rename = "deviceSessionId")]
    pub device_session_id: String,
    #[serde(rename = "globalMultiplayerCorrelationId")]
    pub global_multiplayer_correlation_id: String,
    pub id: PlayerId,
    pub name: String,
    pub pfid: String,
    #[serde(rename = "randomId")]
    pub random_id: u64,
    #[serde(rename = "split-screen-player")]
    pub split_screen_player: bool,
    pub uuid: String,
    pub xuid: String,
    //stats
    #[serde(default)]
    pub avgpacketloss: Option<f64>,
    #[serde(default)]
    pub avgping: Option<i64>,
    #[serde(default)]
    pub maxbps: Option<i64>,
    #[serde(default)]
    pub packetloss: Option<f64>,
    #[serde(default)]
    pub ping: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ListdCustomResult {
    pub success: bool,
    pub is_stats: bool,
    pub player_id: PlayerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListdResult>,
}

pub type ListdResults = Vec<ListdResult>;

pub fn parse_listd_action(json_str: &str) -> Result<ListdAction, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn parse_listd_info(json_str: &str) -> Result<ListdInfo, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn stringify_listd_custom_result(
    result: ListdCustomResult,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&result)
}
