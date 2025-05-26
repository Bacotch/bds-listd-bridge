use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use once_cell::{sync::Lazy};
use regex::Regex;

use super::{color::Color, consts::{LOGLEVEL_SUFFIX, LOG_PREFIX}};

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            LogLevel::Info => Color::White,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Error => Color::Red,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.as_str())
    }
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(level: &str) -> Result<Self, Self::Err> {
        Ok(match level {
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        })
    }
}
static LOGLEVEL_REGEX: Lazy<Regex> =
    Lazy::new(|| {
        let s = format!("{}{}",LOG_PREFIX,LOGLEVEL_SUFFIX);
        Regex::new(&s).expect("Failed to init LOGLEVEL_REGEX")
    });

fn get_log_level(log: &str) -> LogLevel {
    let level = LOGLEVEL_REGEX
        .captures(log)
        .map(|caps| caps["level"].to_string())
        .unwrap_or("INFO".to_string());

    level.parse().unwrap_or(LogLevel::Info)

}

pub fn get_log_color(log: &str) -> Color {
    let level = get_log_level(log);
    level.to_color()
}

pub fn reset_log_color() -> Color {
    Color::Reset
}