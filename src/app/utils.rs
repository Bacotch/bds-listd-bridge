use tokio::sync::mpsc;

pub async fn execute_command(
    tx: &mpsc::Sender<String>,
    command: &str,
) -> Result<(), mpsc::error::SendError<String>> {
    tx.send(format!("{}\n", command.to_string())).await
}
