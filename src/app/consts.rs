pub const LOG_PREFIX: &str = r"^(NO LOG FILE! - )?\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}:\d{3}";
//[Listd] {json}
pub const LISTD_ACTION_PATTERN: &str = r".*\[Scripting\] listd:(?P<json>\{.*\})";
//listdの始点と末端のマーカー。いる？？？
//pub const COMMAND_RESULT_PREFIX: &str = "###*";
//pub const COMMAND_RESULT_SUFFIX: &str = "*###";
//listd本体の解析
pub const LISTD_OUTPUT_PATTERN: &str = r"###\*(?s)(?<json>.*?)\*###";
//[2025-05-25 17:24:39:368 WARN] [Scripting] listd:{"is_stats":true,"player_id":34423423423}
