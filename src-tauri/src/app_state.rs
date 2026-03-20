use std::sync::atomic::{AtomicBool, AtomicU64};

pub static LAST_SHOW_TIME: AtomicU64 = AtomicU64::new(0);
pub static IGNORE_BLUR: AtomicBool = AtomicBool::new(false);
