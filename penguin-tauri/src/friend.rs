use penguin::{FriendList, MessageClient};
use ricq_client::RicqClient;
use tauri::State;

#[tauri::command]
pub async fn get_friend_list(client: State<'_, RicqClient>) -> Result<FriendList, String> {
    Ok(client.get_friend_list().await.unwrap())
}
