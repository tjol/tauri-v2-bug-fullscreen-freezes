use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            app.listen_any("toggle-fullscreen", move |_| {
                let is_fullscreen = main_window.is_fullscreen().unwrap();
                println!("toggle fullscreen  (current: {is_fullscreen})");
                main_window.set_fullscreen(!is_fullscreen).ok();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
