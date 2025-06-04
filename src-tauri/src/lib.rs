use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use dashmap::DashMap;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{GetLastError, HINSTANCE, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Input::KeyboardAndMouse::VIRTUAL_KEY,
            WindowsAndMessaging::{
                CallNextHookEx, DispatchMessageW, PeekMessageW, SetWindowsHookExW,
                TranslateMessage, UnhookWindowsHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG,
                PM_REMOVE, WH_KEYBOARD_LL, WM_KEYDOWN, WM_QUIT, WM_SYSKEYDOWN,
            },
        },
    },
};

// Wrapper around HHOOK to make it Send + Sync
#[derive(Debug, Clone, Copy)]
struct SafeHHook(HHOOK);

unsafe impl Send for SafeHHook {}
unsafe impl Sync for SafeHHook {}

impl SafeHHook {
    fn new(hook: HHOOK) -> Self {
        Self(hook)
    }

    fn get(&self) -> HHOOK {
        self.0
    }
}

static HOOK_HANDLE_FOR_CALLBACK: Mutex<Option<SafeHHook>> = Mutex::new(None);
static LAST_KEY_TIMES: once_cell::sync::Lazy<DashMap<u16, u64>> =
    once_cell::sync::Lazy::new(DashMap::new);
static ENABLE_KEYFIX: once_cell::sync::Lazy<Mutex<bool>> =
    once_cell::sync::Lazy::new(|| Mutex::new(true));
static DEBOUNCE_INTERVAL_MS: once_cell::sync::Lazy<Mutex<u64>> =
    once_cell::sync::Lazy::new(|| Mutex::new(50));

// When this struct is dropped it will signal the hook thread to terminate
struct HookThreadState {
    is_running: Arc<AtomicBool>,
}

impl HookThreadState {
    fn new() -> Self {
        HookThreadState {
            is_running: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl Drop for HookThreadState {
    fn drop(&mut self) {
        // Signal the hook thread to stop when HookThreadState is dropped
        self.is_running.store(false, Ordering::SeqCst);
        println!("HookThreadState dropped, signaled hook thread to stop.");
    }
}

unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let kbd_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
        let vk_code = VIRTUAL_KEY(kbd_struct.vkCode as u16);

        if matches!(w_param.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN) {
            if !ENABLE_KEYFIX.lock().unwrap().clone() {
                return LRESULT(0);
            }

            let debounce_interval_ms = DEBOUNCE_INTERVAL_MS.lock().unwrap();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64;
            let should_block = LAST_KEY_TIMES
                .get(&vk_code.0)
                .map_or(false, |last| now - *last < *debounce_interval_ms);

            if should_block {
                println!("Blocked key: {:?}", vk_code);
                return LRESULT(1);
            } else {
                LAST_KEY_TIMES.insert(vk_code.0, now);
            }
        }
    }

    let stored_hook_handle = HOOK_HANDLE_FOR_CALLBACK.lock().unwrap();
    let hook_handle = stored_hook_handle.map(|h| h.get());
    unsafe { CallNextHookEx(hook_handle, n_code, w_param, l_param) }
}

fn run_keyboard_hook_loop(is_running_arc: Arc<AtomicBool>) -> Result<(), String> {
    println!("Starting global keyboard hook thread.");

    let h_instance: HINSTANCE = unsafe { GetModuleHandleW(PCWSTR::null()) }
        .map_err(|e| format!("Failed to get module handle: {:?}", e))?
        .into();

    if h_instance.is_invalid() {
        return Err("Invalid module handle obtained.".to_string());
    }

    let hook_handle_for_unhook: HHOOK;

    match unsafe {
        SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            Some(h_instance),
            0, // thread id, 0 = global hook
        )
    } {
        Ok(handle) => {
            if handle.is_invalid() {
                let err = unsafe { GetLastError() };
                return Err(format!("Invalid hook handle. OS Error: {:?}", err));
            }
            println!("Hook installed successfully: {:?}", handle);
            *HOOK_HANDLE_FOR_CALLBACK.lock().unwrap() = Some(SafeHHook::new(handle));
            hook_handle_for_unhook = handle; // Store for unhooking
        }
        Err(e) => {
            return Err(format!("Failed to set hook: {:?}", e));
        }
    }

    let mut msg: MSG = MSG::default();
    while is_running_arc.load(Ordering::SeqCst) {
        let result = unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) };
        if result.as_bool() {
            if msg.message == WM_QUIT {
                is_running_arc.store(false, Ordering::SeqCst); // Signal to exit
                break;
            }
            unsafe {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        } else {
            // No message, sleep briefly to avoid busy-waiting.
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    println!("Hook thread signaled to stop. Unhooking...");
    // Unhook
    unsafe {
        if !UnhookWindowsHookEx(hook_handle_for_unhook).is_ok() {
            let unhook_error = GetLastError();
            eprintln!("Failed to unhook keyboard. Error code: {:?}", unhook_error);
        } else {
            println!("Keyboard hook removed successfully.");
        }
        *HOOK_HANDLE_FOR_CALLBACK.lock().unwrap() = None;
    }
    println!("Keyboard hook thread finished.");
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_debounce_interval(interval_ms: u64) -> Result<(), String> {
    if interval_ms < 5 || interval_ms > 1000 {
        return Err("Interval must be between 5 and 1000 milliseconds.".to_string());
    }
    *DEBOUNCE_INTERVAL_MS.lock().unwrap() = interval_ms;
    println!("Debounce interval set to {} milliseconds.", interval_ms);
    Ok(())
}

#[tauri::command]
fn set_keyfix_enabled(enabled: bool) -> Result<(), String> {
    *ENABLE_KEYFIX.lock().unwrap() = enabled;
    println!("Keyfix {} ", if enabled { "enabled" } else { "disabled" });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let hook_thread_state = HookThreadState::new();
    let is_running_clone = hook_thread_state.is_running.clone();

    tauri::Builder::default()
        .manage(hook_thread_state)
        .setup(move |_app| {
            // This thread will run independently of the Tauri main thread.
            std::thread::spawn(move || {
                if let Err(e) = run_keyboard_hook_loop(is_running_clone) {
                    eprintln!("Error in keyboard hook thread: {}", e);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_debounce_interval,
            set_keyfix_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
