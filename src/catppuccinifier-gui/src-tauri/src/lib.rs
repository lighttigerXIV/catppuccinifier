use std::env;

use tauri::Manager;
use view_models::{
    main_vm::{vm_clear_outputs, vm_generate_image, vm_save_image},
    settings_vm::{vm_get_settings, vm_save_settings, vm_set_show_titlebar},
};

pub mod view_models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Prevent Crashes on Wayland
    if cfg!(target_os = "linux") {
        let session_type = env::var("XDG_SESSION_TYPE");

        if let Ok(session_type) = session_type {
            if session_type.to_lowercase() == "wayland" {
                env::set_var("GDK_BACKEND", "wayland");
            }
        }

        env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            vm_get_settings,
            vm_generate_image,
            vm_clear_outputs,
            vm_save_image,
            vm_save_settings,
            vm_set_show_titlebar
        ])
        .setup(|app| {
            let window = app.handle().get_webview_window("main").unwrap();

            window
                .set_decorations(vm_get_settings().unwrap().show_titlebar)
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
