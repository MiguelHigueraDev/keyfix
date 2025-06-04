use crate::keyboard::debounce::{BLOCKED_KEYPRESS_COUNT, DEBOUNCE_INTERVAL_MS, ENABLE_KEYFIX};

#[tauri::command]
pub fn set_debounce_interval(interval_ms: u64) -> Result<(), String> {
    if interval_ms < 5 || interval_ms > 1000 {
        return Err("Interval must be between 5 and 1000 milliseconds.".to_string());
    }
    *DEBOUNCE_INTERVAL_MS.lock().unwrap() = interval_ms;
    println!("Debounce interval set to {} milliseconds.", interval_ms);
    Ok(())
}

#[tauri::command]
pub fn set_keyfix_enabled(enabled: bool) -> Result<(), String> {
    *ENABLE_KEYFIX.lock().unwrap() = enabled;
    println!("Keyfix {} ", if enabled { "enabled" } else { "disabled" });
    Ok(())
}

#[tauri::command]
pub fn get_blocked_keypress_count() -> u64 {
    *BLOCKED_KEYPRESS_COUNT.lock().unwrap()
}
