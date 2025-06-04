use dashmap::DashMap;
use std::{
    sync::{LazyLock, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

pub static LAST_KEY_TIMES: LazyLock<DashMap<u16, u64>> = LazyLock::new(DashMap::new);
pub static ENABLE_KEYFIX: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));
pub static DEBOUNCE_INTERVAL_MS: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(50));
pub static BLOCKED_KEYPRESS_COUNT: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));

pub fn should_block_key(vk_code: u16) -> bool {
    let keyfix_enabled = match ENABLE_KEYFIX.lock() {
        Ok(enabled) => *enabled,
        Err(_) => {
            eprintln!("Failed to acquire ENABLE_KEYFIX lock");
            return false;
        }
    };

    if !keyfix_enabled {
        return false;
    }

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => {
            eprintln!("Failed to get current time");
            return false;
        }
    };

    let debounce_interval = match DEBOUNCE_INTERVAL_MS.lock() {
        Ok(interval) => *interval,
        Err(_) => {
            eprintln!("Failed to acquire DEBOUNCE_INTERVAL_MS lock");
            return false;
        }
    };

    LAST_KEY_TIMES
        .get(&vk_code)
        .map_or(false, |last_time| now - *last_time < debounce_interval)
}

pub fn record_key_press(vk_code: u16) {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => {
            eprintln!("Failed to get current time for recording key press");
            return;
        }
    };

    LAST_KEY_TIMES.insert(vk_code, now);
}

pub fn increment_blocked_count() {
    match BLOCKED_KEYPRESS_COUNT.lock() {
        Ok(mut count) => *count += 1,
        Err(_) => eprintln!("Failed to acquire BLOCKED_KEYPRESS_COUNT lock"),
    }
}
