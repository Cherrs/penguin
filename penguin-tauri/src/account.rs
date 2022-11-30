use penguin::{AccountInfo, MessageClient};
use ricq_client::RicqClient;
use tauri::State;

#[tauri::command]
pub async fn get_account(client: State<'_, RicqClient>) -> Result<AccountInfo, String> {
    Ok(client.get_account().await.unwrap())
}
