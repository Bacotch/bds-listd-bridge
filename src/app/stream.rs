use crate::consts::LOG_PREFIX;
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

static LOG_PREFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(LOG_PREFIX).expect("Failed to init LOG_PREFIX"));

fn remove_newline_suffix(s: &str) -> &str {
    s.strip_suffix("\n").unwrap_or(s)
}

pub struct LogDelimiterStream(mpsc::Receiver<String>);

impl LogDelimiterStream {
    pub fn new(stdout: ChildStdout) {
        let (tx_log, ty_log) = mpsc::channel::<String>(100);
        tokio::spawn(Self::a());
    }
    async fn a() {}
}
