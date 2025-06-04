use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

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

use super::{
    debounce::{increment_blocked_count, record_key_press, should_block_key},
    types::SafeHHook,
};

static HOOK_HANDLE_FOR_CALLBACK: Mutex<Option<SafeHHook>> = Mutex::new(None);

/// Low-level keyboard hook procedure
///
/// This function is called by Windows for every keyboard event in the system.
/// It implements debouncing logic to prevent duplicate key presses.
unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code != HC_ACTION as i32 {
        let stored_hook_handle = HOOK_HANDLE_FOR_CALLBACK.lock().unwrap();
        let hook_handle = stored_hook_handle.map(|h| h.get());
        return unsafe { CallNextHookEx(hook_handle, n_code, w_param, l_param) };
    }

    let kbd_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let vk_code = VIRTUAL_KEY(kbd_struct.vkCode as u16);

    if matches!(w_param.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN) {
        if should_block_key(vk_code.0) {
            increment_blocked_count();
            println!("Blocked key: {:?}", vk_code);
            return LRESULT(1);
        } else {
            record_key_press(vk_code.0);
        }
    }

    // Call the next hook in the chain
    let stored_hook_handle = HOOK_HANDLE_FOR_CALLBACK.lock().unwrap();
    let hook_handle = stored_hook_handle.map(|h| h.get());
    unsafe { CallNextHookEx(hook_handle, n_code, w_param, l_param) }
}

/// Install and run the keyboard hook in a message loop
pub fn run_keyboard_hook_loop(is_running_arc: Arc<AtomicBool>) -> Result<(), String> {
    println!("Starting global keyboard hook thread.");

    // Get the module handle for the current process
    let h_instance: HINSTANCE = unsafe { GetModuleHandleW(PCWSTR::null()) }
        .map_err(|e| format!("Failed to get module handle: {:?}", e))?
        .into();

    if h_instance.is_invalid() {
        return Err("Invalid module handle obtained.".to_string());
    }

    let hook_handle = match unsafe {
        SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            Some(h_instance),
            0, // thread id, (0 = global hook)
        )
    } {
        Ok(handle) => {
            if handle.is_invalid() {
                let err = unsafe { GetLastError() };
                return Err(format!("Invalid hook handle. OS Error: {:?}", err));
            }
            println!("Hook installed successfully: {:?}", handle);

            *HOOK_HANDLE_FOR_CALLBACK.lock().unwrap() = Some(SafeHHook::new(handle));
            handle
        }
        Err(e) => {
            return Err(format!("Failed to set hook: {:?}", e));
        }
    };

    let result = run_message_loop(is_running_arc);
    cleanup_hook(hook_handle);

    result
}

/// Run the Windows message loop for processing hook events
fn run_message_loop(is_running_arc: Arc<AtomicBool>) -> Result<(), String> {
    let mut msg: MSG = MSG::default();

    while is_running_arc.load(Ordering::SeqCst) {
        let result = unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) };

        if result.as_bool() {
            if msg.message == WM_QUIT {
                println!("Received WM_QUIT message, exiting hook loop");
                is_running_arc.store(false, Ordering::SeqCst);
                break;
            }

            unsafe {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        } else {
            // No message available, sleep briefly to avoid busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    println!("Message loop finished normally");
    Ok(())
}

/// Clean up the keyboard hook
fn cleanup_hook(hook_handle: HHOOK) {
    println!("Hook thread signaled to stop. Unhooking...");

    unsafe {
        if UnhookWindowsHookEx(hook_handle).is_err() {
            let unhook_error = GetLastError();
            eprintln!("Failed to unhook keyboard. Error code: {:?}", unhook_error);
        } else {
            println!("Keyboard hook removed successfully.");
        }

        // Clear the stored hook handle
        *HOOK_HANDLE_FOR_CALLBACK.lock().unwrap() = None;
    }

    println!("Keyboard hook thread finished.");
}
