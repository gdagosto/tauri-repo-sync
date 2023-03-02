#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod utils;
mod commands;
mod structs;




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::run_shell,
            commands::check_for_updates,
            commands::try_to_pull
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
