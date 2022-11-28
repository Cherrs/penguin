#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod account;
mod friend;
mod login;

use login::StateLoop;
use penguin::ricq_client::RicqClient;
use std::sync::Mutex;
#[cfg(debug_assertions)]
use tauri::Manager;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // 订阅trace
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // 初始化RicqClient
    let client = RicqClient::init().await;
    tauri::Builder::default()
        // 使用tauri状态管理共享client
        .manage(client)
        .manage(StateLoop(Mutex::new(false)))
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            login::get_qrcode,
            login::get_qrcode_state,
            login::start_get_qrcode_state,
            login::get_qrcode_state_loop,
            account::get_account,
            friend::get_friend_list
        ])
        // debug环境下打开开发者工具
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
