mod commands;
mod keyboard;
mod state;
mod tray;

use commands::*;
use keyboard::run_keyboard_hook_loop;
use single_instance::SingleInstance;
use state::HookThreadState;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use tauri::Manager;
use tray::{handle_tray_event, setup_tray};

const FOCUS_PORT: u16 = 49523;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let instance = SingleInstance::new("keyfix_lock").unwrap();

    if !instance.is_single() {
        // Notify the running instance before exiting to focus on it
        let _ = TcpStream::connect(("127.0.0.1", FOCUS_PORT));
        std::process::exit(0);
    }

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

            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                if let Ok(listener) = TcpListener::bind(("127.0.0.1", FOCUS_PORT)) {
                    println!("Focus listener started on port {}", FOCUS_PORT);

                    for stream in listener.incoming() {
                        match stream {
                            // Focus window when another window is opened
                            Ok(mut stream) => {
                                let mut buffer = [0; 1024];
                                let _ = stream.read(&mut buffer);

                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    let _ = window.unminimize();
                                } else {
                                    // Shouldn't happen but try to focus another window
                                    if let Some(window) =
                                        app_handle.webview_windows().values().next()
                                    {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                        let _ = window.unminimize();
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error accepting TCP connection: {}", e);
                            }
                        }
                    }
                } else {
                    eprintln!("Failed to bind TCP listener on port {}", FOCUS_PORT);
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
