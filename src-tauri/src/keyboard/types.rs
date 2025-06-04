use windows::Win32::UI::WindowsAndMessaging::HHOOK;

// Wrapper around HHOOK to make it Send + Sync
#[derive(Debug, Clone, Copy)]
pub struct SafeHHook(HHOOK);

unsafe impl Send for SafeHHook {}
unsafe impl Sync for SafeHHook {}

impl SafeHHook {
    pub fn new(hook: HHOOK) -> Self {
        Self(hook)
    }

    pub fn get(&self) -> HHOOK {
        self.0
    }
}
