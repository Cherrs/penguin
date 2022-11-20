#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod login;

use penguin::ricq::RicqClient;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let client = RicqClient::init().await;
    tauri::Builder::default()
        //使用tauri状态管理共享client
        .manage(client)
        .invoke_handler(tauri::generate_handler![greet, login::get_qrcode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
