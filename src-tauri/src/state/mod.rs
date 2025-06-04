use std::sync::{atomic::AtomicBool, Arc};

pub struct HookThreadState {
    pub is_running: Arc<AtomicBool>,
}

impl HookThreadState {
    pub fn new() -> Self {
        HookThreadState {
            is_running: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl Drop for HookThreadState {
    fn drop(&mut self) {
        self.is_running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        println!("HookThreadState dropped, signaled hook thread to stop.");
    }
}
