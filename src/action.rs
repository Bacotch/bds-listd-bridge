use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct ListdActionPayload {
    pub is_stats: bool,
    pub player_id: String,
}

//###* {"command":"listd","result":[]}
//*###

#[derive(Deserialize)]
pub struct ListdOutPutPayload {
    pub command: String, //"listd"固定。一応解析した文字列がただしいのかどうかを確かめるために使う？？
    pub result: Vec<ListdResult>, //ListdResult[]と同義
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
    pub id: String, //内部的にはnumberだが、APIではstring型とされている。型変換としてSringを指定しているが問題が起きるなら別の方法を考える
    pub name: String,
    pub pfid: String,
    #[serde(rename = "randomId")]
    pub random_id: i32,
    #[serde(rename = "split-screen-player")]
    pub split_screen_player: bool,
    pub uuid: String,
    pub xuid: String,
}
//serde renameについては謎

pub fn parse_listd_action(json_str: &str) -> Result<ListdActionPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn parse_listd_output(json_str: &str) -> Result<ListdOutPutPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}

//
