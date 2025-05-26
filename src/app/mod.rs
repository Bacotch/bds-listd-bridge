mod listd_types;
mod consts;
mod listd_manager;
mod log_parser;
mod stream;
mod utils;
use listd_types::{ListdAction, ListdResults};
use listd_manager::ListdManager;
use log_level::{get_log_color, reset_log_color};
use log_parser::{LogParser, LogType};
use std::path::Path;
use std::process::Stdio;
use stream::LogDelimiterStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};
use tokio::signal;
use tokio::sync::mpsc;
mod color;
mod log_level;

pub struct App {
    cwd: String,
    executable_name: String,
}

impl App {
    pub fn new(cwd: String, executable_name: String) -> Self {
        App {
            cwd,
            executable_name,
        }
    }

    pub async fn run(self) {
        let full_path = Path::new(&self.cwd).join(&self.executable_name);
        let mut child = Command::new(&full_path)
            .current_dir(&self.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process.");

        let child_stdin = child.stdin.take().expect("Failed to take child.stdin");
        let child_stdout = child.stdout.take().expect("Failed to take child.stdout");

        let (tx, rx) = mpsc::channel::<String>(100);

        let command_tx = tx.clone();

        tokio::spawn(Self::handle_user_input(tx));

        tokio::spawn(Self::handle_child_stdin(rx, child_stdin));

        tokio::spawn(Self::handle_child_stdout(command_tx, child_stdout));

        tokio::select! {
            _= signal::ctrl_c() => {
                if let Err(e) = child.kill().await {
                    eprintln!("ERROR: Failed to kill child process: {}", e);
                }
            }
            status = child.wait()=> {
                match status {
                    Ok(s) => println!("Child process exited with status:{}", s),
                    Err(e) => eprintln!("ERROR: Child process failed to exit: {}", e)
                }
            }
        }
        println!("Press any key to continue...");
    }

    //1.キーボード入力をreceiverに送る。
    async fn handle_user_input(tx: mpsc::Sender<String>) {
        //親プロセスの標準入力をbufferでラップして取得
        let mut stdin_reader = BufReader::new(tokio::io::stdin());
        let mut line = String::new();
        loop {
            let bytes_read_res = stdin_reader.read_line(&mut line).await;
            match bytes_read_res {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        //EOF
                        break;
                    }
                    if let Err(e) = tx.send(line.clone()).await {
                        eprintln!("Failed to send line to channel (receiver dropped?): {}", e);
                        break;
                    }
                    line.clear();
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::Interrupted => {
                        eprintln!("Stdin was interrupted: {}", e);
                        line.clear();
                        continue;
                    }
                    _ => {
                        eprintln!("Fatal stdin error, closing input task: {}", e);
                        break;
                    }
                },
            }
        }
    }

    async fn handle_child_stdin(mut rx: mpsc::Receiver<String>, child_stdin: ChildStdin) {
        let mut child_stdin = child_stdin;

        while let Some(input_line) = rx.recv().await {
            if let Err(e) = child_stdin.write_all(input_line.as_bytes()).await {
                eprintln!("Failed to write to child stdin: {}", e);
                break;
            }
        }
    }

    async fn handle_child_stdout(command_tx: mpsc::Sender<String>, child_stdout: ChildStdout) {
        let mut stream = LogDelimiterStream::new(child_stdout);
        let (log_tx, log_rx) = mpsc::channel::<String>(100);

        tokio::spawn(async move {
            while let Some(entry) = stream.next().await {
                if let Err(e) = log_tx.send(entry).await {
                    eprintln!("App: Failed to send log entry to parser channel: {}", e);
                    break;
                }
            }
        });

        let (logtype_tx, mut logtype_rx) = mpsc::channel::<LogType>(50);
        let parser = LogParser::new(logtype_tx);
        tokio::spawn(parser.run(log_rx));

        let (results_tx, results_rx) = mpsc::channel::<ListdResults>(50);
        let (action_tx, action_rx) = mpsc::channel::<ListdAction>(50);
        let manager: ListdManager = ListdManager::new(command_tx);
        tokio::spawn(manager.run(action_rx, results_rx));

        while let Some(logtype) = logtype_rx.recv().await {
            match logtype {
                LogType::ListdResults(results) => {
                    if let Err(e) = results_tx.send(results).await {
                        eprintln!("an error occured {}", e)
                    }
                }
                LogType::ListdAction(action) => {
                    if let Err(e) = action_tx.send(action).await {
                        eprintln!("an error occured {}", e)
                    }
                }
                LogType::Unknown(log) => {
                    println!("{}", log.trim_end());
                }
                LogType::Regular(log) => {
                    println!("{}{}{}", get_log_color(&log),log.trim_end(),reset_log_color());
                }
            }
        }
    }
}
