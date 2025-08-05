mod app;
use app::App;
use std::env;
use tokio::main;
#[main]
async fn main() {
    let os = env::consts::OS;
    let cwd = env::args().nth(1).unwrap_or(".".to_string());
      let default_executable_name = match os {
        "windows" => "bedrock_server.exe",
        "linux" => "bedrock_server",
        _ => panic!("Unsupported operating system: {}", os)
    };
    let executable_name = env::args().nth(2).unwrap_or(default_executable_name.to_string());
    let app = App::new(cwd, executable_name,os.to_string());
    app.run().await
}
