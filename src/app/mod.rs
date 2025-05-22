mod stream;
use std::path::Path;
use std::process::Stdio;
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
            //あとで読むの失敗したくらいで終了しないようにハンドリングしておく
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
    async fn handle_child_stdout(child_stdout: ChildStdout) {
        //子プロセスの出力をBufReaderでラップ
        let mut child_stdout_reader = BufReader::new(child_stdout);
        //lineを初期化
        let mut line = String::new();

        loop {
            let bytes_read = child_stdout_reader
                .read_line(&mut line)
                .await
                .expect("Failed to read from child stdout");
            if bytes_read == 0 {
                //read_lineがbyte0で返すときは子プロセスが終了したことを表す
                break;
            }
            println!("{}", line.trim_end());
            //lineを空にする
            line.clear();
        }
    }
}
