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
    pub fn new(stdout: ChildStdout) -> Self {
        let (tx, rx) = mpsc::channel::<String>(100);
        tokio::spawn(Self::run(stdout, tx));
        Self(rx)
        //;を記述すると式ではなく文として評価されるためreturnされないことに注意
    }
    async fn run(stdout: ChildStdout, tx: Sender<String>) {
        let mut reader = BufReader::new(stdout);
        let mut line_buffer = String::new();
        let mut current_log_message_buffer = String::new(); //ログのひとまとまり

        let idle_timeout = Duration::from_millis(50);

        loop {
            line_buffer.clear(); //line_bufferを初期化

            tokio::select! {
                //read_resはbyte,line_bufferはString,current_log_message_bufferは最後にぶち込まれる
                //pattern1 タイムアウトせずに読めた時
                read_byte = reader.read_line(&mut line_buffer) => {//?
                    match read_byte {
                        //EOF
                        Ok(0) => {
                            if !current_log_message_buffer.is_empty() {
                                if let Err(e) = tx.send(remove_newline_suffix(&current_log_message_buffer).to_string()).await {
                                    eprintln!("LogDelimiterStream: Error sending remaining buffer on EOF:  {}",e)
                                }
                            }
                            break;

                        },
                        //読み込みおｋ
                        Ok(_) => {
                            //log prefixに合致するか確認する
                            if LOG_PREFIX_REGEX.is_match(&line_buffer) && !current_log_message_buffer.is_empty() {
                                if let Err(e) = tx.send(remove_newline_suffix(&current_log_message_buffer).to_string()).await {
                                  eprintln!("LogDelimiterStream: Error sending bufferd log: {}",e);
                                  break;
                                }
                                current_log_message_buffer.clear();
                            }
                            //lineをおおきいbufferに積んでいく
                            current_log_message_buffer.push_str(&line_buffer);
                        },

                        //Error時。clearしてcontinueすべき？
                        Err(e)=>{
                            eprintln!("LogDelimiterStream: Error reading from child stdout: {}",e);
                            break;
                        }
                    }
                },
                //pattern2 タイムアウト時
                _ = sleep(idle_timeout), if !current_log_message_buffer.is_empty() => {
                    //タイムアウトしててもバッファにデータがあるなら送信する
                    if let Err(e) = tx.send(remove_newline_suffix(&current_log_message_buffer).to_string()).await {
                        eprintln!("LogDelimiterStream: Error sending buffered log due to timeout: {}",e);
                    }
                    current_log_message_buffer.clear();
                }
            }
        }
    }

    //receiverが受け取った要素を取り出す為に使う。
    pub async fn next(&mut self) -> Option<String> {
        self.0.recv().await
    }
}
