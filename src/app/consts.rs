pub const LOG_PREFIX: &str = r"^(NO LOG FILE! - )?\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}:\d{3}";
pub const LISTD_ACTION_PATTERN: &str = r".*\[Scripting\] listd:(?P<json>\{.*\})";
pub const LISTD_OUTPUT_PATTERN: &str = r"###\*(?s)(?<json>.*?)\*###";
