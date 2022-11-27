use penguin::{ricq_client::RicqClient, AccountInfo};
use tauri::State;

#[tauri::command]
pub async fn get_account(client: State<'_, RicqClient>) -> Result<AccountInfo, String> {
    Ok(client.get_account().await.unwrap())
}
