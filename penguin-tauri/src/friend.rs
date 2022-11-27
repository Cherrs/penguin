use penguin::{ricq_client::RicqClient, FriendList};
use tauri::State;

#[tauri::command]
pub async fn get_friend_list(client: State<'_, RicqClient>) -> Result<FriendList, String> {
    Ok(client.get_friend_list().await.unwrap())
}
