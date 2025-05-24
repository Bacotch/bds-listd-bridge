pub const LOG_PREFIX: &str = r"^(NO LOG FILE! - )?\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}:\d{3}";
//[Listd] {json}
pub const LISTD_ACTION_PATTERN: &str = r"\[Listd\] (?P<json>\{.*\})";
//listdの始点と末端のマーカー。いる？？？
//pub const COMMAND_RESULT_PREFIX: &str = "###*";
//pub const COMMAND_RESULT_SUFFIX: &str = "*###";
//listd本体の解析
pub const LISTD_OUTPUT_PATTERN: &str = r"###\*(?P<json>.*)\*###";
