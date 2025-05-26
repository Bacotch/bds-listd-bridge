use super::action::{parse_listd_action, parse_listd_info, ListdAction, ListdResults};
use super::consts::{LISTD_ACTION_PATTERN, LISTD_OUTPUT_PATTERN};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::sync::mpsc;

static LISTD_ACTION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(LISTD_ACTION_PATTERN).expect("Failed to init LISTD_ACTION_PATTERN"));

static LISTD_OUTPUT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(LISTD_OUTPUT_PATTERN).expect("Failed to init LISTD_OUTPUT_PATTERN"));

pub enum LogType {
    Regular(String),
    Unknown(String),
    ListdAction(ListdAction),
    ListdResults(ListdResults),
}

pub struct LogParser(mpsc::Sender<LogType>);

impl LogType {
    pub fn parse(log: String) -> Self {
        if let Some(payload) = LISTD_OUTPUT_REGEX
            .captures(&log)
            .and_then(|caps| caps.name("json"))
            .and_then(|json_match| {
                let json_str = json_match.as_str();
                parse_listd_info(json_str).ok()
            })
        {
            if payload.command == "listd" {
                if payload.result.is_empty() {
                    return LogType::Unknown(log);
                } else {
                    let listd_results = payload.result;
                    return LogType::ListdResults(listd_results);
                }
            } else {
                return LogType::Unknown(log);
            }
        } else if let Some(payload) = LISTD_ACTION_REGEX
            .captures(&log)
            .and_then(|caps| caps.name("json"))
            .and_then(|json_match| {
                let json_str = json_match.as_str();
                parse_listd_action(json_str).ok()
            })
        {
            return LogType::ListdAction(payload);
        }
        return LogType::Regular(log);
    }
}

impl LogParser {
    pub fn new(tx: mpsc::Sender<LogType>) -> Self {
        LogParser(tx)
    }

    pub async fn run(self, mut rx: mpsc::Receiver<String>) {
        while let Some(log) = rx.recv().await {
            let parsed_log_type = LogType::parse(log);
            if let Err(e) = self.0.send(parsed_log_type).await {
                eprintln!("LogParser: Failed to send parsed log: {}", e);
                break;
            }
        }
    }
}
