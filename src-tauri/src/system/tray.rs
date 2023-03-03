use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn build_tray() -> SystemTray {
    // Create system tray items
    let configure = CustomMenuItem::new("configure".to_string(), "Configure");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // Build the menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(configure)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // Build the tray
    SystemTray::new().with_menu(tray_menu)
}



pub fn build_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "configure" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
