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
    Regular(String),            //そのまま出力
    Unknown(String),            //そのまま出力
    ListdAction(ListdAction),   //[listd]{json}の形のやつ、出力しない
    ListdResults(ListdResults), //###で始まる奴、出力しない。これだけだと別の内容も拾う可能性があるのでもう少し汎用にする？
}

pub struct LogParser(mpsc::Sender<LogType>);

//ログ解析をしてLogTypeを返す(?)
//このログが表示されるべきかも返す
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
                    println!("Unknown");
                    return LogType::Unknown(log);
                } else {
                    let listd_results = payload.result; //resultプロパティを代入。
                    println!("Results");
                    return LogType::ListdResults(listd_results);
                }
                //解析した結果のcommandプロパティがlistdであるかどうかを確認
            } else {
                println!("Unknown");
                return LogType::Unknown(log); //可能性はあるので。別にエラーではない。
            }
        } else if let Some(payload) = LISTD_ACTION_REGEX
            .captures(&log)
            .and_then(|caps| caps.name("json"))
            .and_then(|json_match| {
                let json_str = json_match.as_str();
                parse_listd_action(json_str).ok()
            })
        {
            println!("Action");
            return LogType::ListdAction(payload);
        }
        //どのifにも引っかからなかったならそれは普通のログ
        println!("Regular");
        return LogType::Regular(log);
    }
}

//生ログを処理してLogTypeに分類して送信する
impl LogParser {
    //LogtypeSenderを受け取ってるのは使用側でreceiverを使って表示処理を行うため。こちらは送信するだけ。
    pub fn new(tx: mpsc::Sender<LogType>) -> Self {
        LogParser(tx)
    }
    //生logを受け取るためのString Receiverを受け取って、自身のLogtypeSenderでLogTypeReceiverに送信する
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
