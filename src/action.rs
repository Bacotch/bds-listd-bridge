use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct ListdActionPayload {
    pub is_stats: bool,
    pub player_id: String,
}

pub fn parse_listd_action(json_str: &str) -> Result<ListdActionPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}
