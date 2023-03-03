#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod utils;
mod commands;
mod structs;
mod system;




fn main() {
    tauri::Builder::default()
        .system_tray(system::build_tray())
        .on_system_tray_event(|app, event| system::build_tray_events(app, event))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}

        })
        .invoke_handler(tauri::generate_handler![
            commands::run_shell,
            commands::check_for_updates,
            commands::try_to_pull
        ])
        
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
              api.prevent_exit();
            }
            _ => {}
          });
}
