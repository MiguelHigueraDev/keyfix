mod commands;
mod keyboard;
mod state;
mod tray;

use commands::*;
use keyboard::run_keyboard_hook_loop;
use state::HookThreadState;
use tray::{handle_tray_event, setup_tray};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let hook_thread_state = HookThreadState::new();
    let is_running_clone = hook_thread_state.is_running.clone();

    tauri::Builder::default()
        .manage(hook_thread_state)
        .setup(move |app| {
            std::thread::spawn(move || {
                if let Err(e) = run_keyboard_hook_loop(is_running_clone) {
                    eprintln!("Error in keyboard hook thread: {}", e);
                }
            });

            setup_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .on_tray_icon_event(handle_tray_event)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_debounce_interval,
            set_keyfix_enabled,
            get_blocked_keypress_count
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
