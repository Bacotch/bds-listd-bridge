use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{mpsc, oneshot, Mutex};

use crate::app::utils::execute_command;

use super::listd_types::{
    stringify_listd_custom_result, ListdAction, ListdCustomResult, ListdResults, PlayerId,
};
pub struct ListdManager {
    bds_stdin_tx: mpsc::Sender<String>,
    requests: Mutex<HashMap<RequestKey, oneshot::Sender<ListdCustomResult>>>,
}
#[derive(PartialEq, Eq, Hash, Clone)]
struct RequestKey {
    pub player_id: PlayerId,
    pub is_stats: bool,
}

impl ListdManager {
    pub fn new(tx: mpsc::Sender<String>) -> Self {
        let requests = HashMap::new();
        ListdManager {
            bds_stdin_tx: tx,
            requests: Mutex::new(requests),
        }
    }
    pub async fn run(
        self,
        action_rx: mpsc::Receiver<ListdAction>,
        info_rx: mpsc::Receiver<ListdResults>,
    ) {
        let manager = Arc::new(self);
        let action_manager = manager.clone();
        tokio::spawn(async move { action_manager.handle_listd_action(action_rx).await });
        let info_manager = manager.clone();
        tokio::spawn(async move { info_manager.handle_listd_info(info_rx).await });
    }
    async fn handle_listd_action(self: Arc<Self>, mut rx: mpsc::Receiver<ListdAction>) {
        while let Some(action) = rx.recv().await {
            let request_key = RequestKey {
                player_id: action.player_id.clone(),
                is_stats: action.is_stats,
            };
            let request_key_spawn = request_key.clone();
            let is_stats = action.is_stats;
            let listd = if is_stats { "listd stats" } else { "listd" };

            let (tx, rx_oneshot) = oneshot::channel::<ListdCustomResult>();
            {
                let mut requests_guard = self.requests.lock().await;
                requests_guard.insert(request_key, tx);
            }

            if let Err(e) = execute_command(&self.bds_stdin_tx, &listd).await {
                eprintln!("Failed to send message. {}", e);
                let mut requests_guard = self.requests.lock().await;
                requests_guard.remove(&request_key_spawn);
                continue;
            }

            let manager = self.clone();
            let player_id = action.player_id;
            tokio::spawn(async move {
                let timeout_duration = Duration::from_secs(5);

                match tokio::time::timeout(timeout_duration, rx_oneshot).await {
                    Ok(Ok(result)) => {
                        if let Some(serialized) = stringify_listd_custom_result(result).ok() {
                            let scriptevent = format!("scriptevent listd:result {}", serialized);
                            if let Err(e) =
                                execute_command(&manager.bds_stdin_tx, &scriptevent).await
                            {
                                eprintln!("Failed to send message. {}", e);
                            }
                        }
                    }
                    Ok(Err(_)) => {
                        //sender dropped
                        eprintln!(
                            "Task: Oneshot sender for player {} was dropped unexpectedly.",
                            player_id
                        );
                        let mut requests_guard = manager.requests.lock().await;
                        requests_guard.remove(&request_key_spawn);
                    }
                    Err(_) => {
                        //timeout
                        eprintln!(
                            "Task: Timeout waiting for listd result for player {}",
                            player_id
                        );
                        let mut requests_guard = manager.requests.lock().await;
                        requests_guard.remove(&request_key_spawn);
                        let result = ListdCustomResult {
                            success: false,
                            player_id: request_key_spawn.player_id.clone(),
                            is_stats: request_key_spawn.is_stats,
                            data: None,
                        };
                        if let Some(serialized) = stringify_listd_custom_result(result).ok() {
                            let scriptevent = format!("scriptevent listd:result {}", serialized);
                            if let Err(e) =
                                execute_command(&manager.bds_stdin_tx, &scriptevent).await
                            {
                                eprintln!(
                                    "Task: Failed to send timeout scriptevent for player {}: {}",
                                    player_id, e
                                );
                            };
                        }
                    }
                }
            });
        }
    }

    async fn handle_listd_info(self: Arc<Self>, mut rx: mpsc::Receiver<ListdResults>) {
        while let Some(results) = rx.recv().await {
            let is_stats = results[0].ping.is_some();
            for result in results {
                let player_id = result.id.clone();
                let request_key = RequestKey {
                    player_id: player_id,
                    is_stats: is_stats,
                };
                let mut requests_guard = self.requests.lock().await;
                if let Some(tx) = requests_guard.remove(&request_key) {
                    drop(requests_guard);
                    let custom_result = ListdCustomResult {
                        success: true,
                        player_id: request_key.player_id,
                        is_stats: request_key.is_stats,
                        data: Some(result),
                    };
                    if let Err(e) = tx.send(custom_result) {
                        eprintln!("Output: Non-stats request for player: {}", e.player_id)
                    }
                }
            }
        }
    }
}
