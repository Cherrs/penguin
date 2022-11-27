use std::sync::Mutex;
use std::time::Duration;

use penguin::ricq_client::RicqClient;
use penguin::LoginState;
use tauri::State;
use tauri::Window;

#[tauri::command]
pub async fn get_qrcode(client: State<'_, RicqClient>) -> Result<String, ()> {
    Ok(client.get_login_qrcode_base64().await.unwrap())
}

#[tauri::command(async)]
pub async fn get_qrcode_state(client: State<'_, RicqClient>) -> Result<LoginState, String> {
    match client.get_qrcode_state().await {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

pub struct StateLoop(pub Mutex<bool>);

/// 开始获取登录状态
#[tauri::command(async)]
pub async fn start_get_qrcode_state(
    client: State<'_, RicqClient>,
    state_loop: State<'_, StateLoop>,
    window: Window,
) -> Result<(), String> {
    if *state_loop.0.lock().unwrap() {
        return Err("已经开始".into());
    }
    *state_loop.0.lock().unwrap() = true;
    loop {
        match client.get_qrcode_state().await {
            Ok(state) => {
                window.emit("qrcode_state", &state).unwrap();
                if let LoginState::Confirmed(_) = state {
                    return Ok(());
                }
            }
            Err(e) => {
                window.emit("qrcode_state", e.to_string()).unwrap();
            }
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
/// 状态获取是否开始
#[tauri::command(async)]
pub async fn get_qrcode_state_loop(state_loop: State<'_, StateLoop>) -> Result<bool, String> {
    Ok(*state_loop.0.lock().unwrap())
}
