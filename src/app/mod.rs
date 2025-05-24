mod log_parser;
mod stream;
use log_parser::{LogParser, LogType};
use std::path::Path;
use std::process::Stdio;
use stream::LogDelimiterStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};
use tokio::sync::mpsc;

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
        //bedrock_server.exeのパス
        let full_path = Path::new(&self.cwd).join(&self.executable_name);
        //Command::newで新しい子プロセスを起動。その後のメソッドは詳細設定
        let mut child = Command::new(&full_path)
            .current_dir(&self.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process.");

        let child_stdin = child.stdin.take().expect("Failed to take child.stdin");
        let child_stdout = child.stdout.take().expect("Failed to take child.stdout");

        //tx:sender,rx:receiver
        let (tx, rx) = mpsc::channel::<String>(100);

        //let log_action_tx = tx.clone();

        //各タスクを起動
        tokio::spawn(Self::handle_user_input(tx));

        tokio::spawn(Self::handle_child_stdin(rx, child_stdin));

        tokio::spawn(Self::handle_child_stdout(child_stdout));

        //終了コードを待つ
        let status = child.wait().await.expect("Child process failed to exit.");
        println!("Child process exited with status:{}", status);
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

    //2.receiverから子プロセスのstdinに送る。
    //また、任意のsenderからreceiver->child_stdinを行うためにも利用される。
    async fn handle_child_stdin(mut rx: mpsc::Receiver<String>, child_stdin: ChildStdin) {
        let mut child_stdin = child_stdin;
        //子プロセスの標準入力を取得
        //tx->rxの受け渡し。receiverが受け取ったString(senderから送られるのはline)をinput_lineで受け取ってる
        while let Some(input_line) = rx.recv().await {
            if let Err(e) = child_stdin.write_all(input_line.as_bytes()).await {
                eprintln!("Failed to write to child stdin: {}", e);
                break;
            }
        }
    }

    //3.子プロセスの出力をprintlfで表示する。1と2と3でこのプログラムを挟まないのと同じ動作を実現する。
    //LogDelimiterとLogParserが存在する
    //LogParserが解析すべきログであった場合にprintlnされないようにする
    async fn handle_child_stdout(child_stdout: ChildStdout) {
        let mut stream = LogDelimiterStream::new(child_stdout);
        let (log_tx, log_rx) = mpsc::channel::<String>(100);
        let (logtype_tx, mut logtype_rx) = mpsc::channel::<LogType>(100);
        tokio::spawn(async move {
            while let Some(entry) = stream.next().await {
                if let Err(e) = log_tx.send(entry).await {
                    eprintln!("App: Failed to send log entry to parser channel: {}", e);
                    break;
                }
            }
        });

        let parser = LogParser::new(logtype_tx);
        tokio::spawn(parser.run(log_rx));

        while let Some(logtype) = logtype_rx.recv().await {
            match logtype {
                LogType::ListdOutput(payload) => {
                    println!()
                }
                LogType::ListdAction(payload) => {
                    println!()
                }
                LogType::Unknown(log) => {
                    println!("{}", log.trim_end());
                }
                LogType::Regular(log) => {
                    println!("{}", log.trim_end());
                }
            }
        }
    }
}
