use penguin::ricq::RicqClient;
use tauri::State;

#[tauri::command]
pub async fn get_qrcode(client: State<'_, RicqClient>) -> Result<String, ()> {
    Ok(client.get_login_qrcode_base64().await.unwrap())
}
