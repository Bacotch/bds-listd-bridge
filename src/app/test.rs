#[cfg(test)]
mod tests {
    use crate::{
        action::{parse_listd_action, parse_listd_info},
        consts::{LISTD_ACTION_PATTERN, LISTD_OUTPUT_PATTERN},
    };
    use once_cell::sync::Lazy;
    use regex::Regex;
    #[test]
    fn test() {
        static LISTD_ACTION_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(LISTD_ACTION_PATTERN).expect("Failed to init LISTD_ACTION_PATTERN")
        });

        let test = "[2025-05-25 17:24:39:368 WARN] [Scripting] listd:{\"isStats\": true, \"playerId\": 34423423423}";
        if let Some(action) = LISTD_ACTION_REGEX
            .captures(test)
            .and_then(|caps| caps.name("json"))
            .and_then(|json_match| {
                let json_str = json_match.as_str();
                parse_listd_action(json_str).ok()
            })
        {
            println!("Action,{},{}", action.is_stats, action.player_id)
        } else {
            println!("Regular")
        }
    }
    #[test]
    fn test2() {
        static LISTD_OUTPUT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(LISTD_OUTPUT_PATTERN).expect("Failed to init LISTD_OUTPUT_PATTERN")
        });
        let test = r#"[2025-05-25 22:27:57:120 INFO] There are 1/10 players online:
TestPlayer2345032490
###* {"command":"listd","result":[{"activeSessionId":"8f1b2bb7-bb90-4d05-9dfb-459480d48a79","clientId":"raknet:12410558830150752769","color":"ffededed","deviceSessionId":"c5d8f52f-ab92-373c-8f91-a9ff1f5b995b","globalMultiplayerCorrelationId":"<raknet>9e0c-7a97-2002-ff55","id":-197568495495,"name":"ACnoawer24","pfid":"f8f3fe0c2919b90b","randomId":10088606783790342701,"split-screen-player":false,"uuid":"b12818c7-d92b-3b77-a82f-65f842d5f08a","xuid":"2535436881684018"}]}
 *###"#;
        //println!("{}", &test);
        if let Some(output) = LISTD_OUTPUT_REGEX
            .captures(test)
            .and_then(|caps| caps.name("json"))
            .and_then(|json_match| {
                let json_str = json_match.as_str();
                match parse_listd_info(json_str) {
                    Ok(parsed) => {
                        println!("Ok");
                        Some(parsed)
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            })
        {
            println!("info");
        } else {
            println!("Regular")
        }
    }
}
