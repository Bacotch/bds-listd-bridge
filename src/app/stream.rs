use crate::consts::LOG_PREFIX;
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;
use tokio::sync::mpsc::{self, Sender};
use tokio::time::{sleep, Duration};

static LOG_PREFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(LOG_PREFIX).expect("Failed to init LOG_PREFIX"));

fn remove_newline_suffix(s: &str) -> &str {
    s.strip_suffix("\n").unwrap_or(s)
}

pub struct LogDelimiterStream(mpsc::Receiver<String>);

impl LogDelimiterStream {
    pub fn new(stdout: ChildStdout) {
        let (tx, rx) = mpsc::channel::<String>(100);
        tokio::spawn(Self::run(stdout, tx));
        Self(rx);
    }
    fn run(stdout: ChildStdout, tx: Sender<String>) {
        let mut reader = BufReader::new(stdout);
        let mut line_buffer = String::new();
    }
}
