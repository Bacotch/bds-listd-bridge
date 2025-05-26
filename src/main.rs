mod app;
use app::App;
use std::env;
use tokio::main;
#[main]
async fn main() {
    //cargo run の第一引数にパスが与えられたらそこをcurrentWorkingDirectoryにする。未指定で./
    let cwd = env::args().nth(1).unwrap_or(".".to_string());
    //cargo run の第二引数にパスが与えられたらそこをbedrock_server.exe本体とする。未指定で同フォルダ。
    let executable_name = env::args()
        .nth(2)
        .unwrap_or("bedrock_server.exe".to_string());
    let app = App::new(cwd, executable_name);
    app.run().await
}
