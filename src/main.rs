mod app;
use app::App;
use std::env;
use tokio::main;
#[main]
async fn main() {
    let cwd = env::args().nth(1).unwrap_or(".".to_string());
    let executable_name = env::args()
        .nth(2)
        .unwrap_or("bedrock_server.exe".to_string());
    let app = App::new(cwd, executable_name);
    app.run().await
}
