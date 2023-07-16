rust
use std::thread::JoinHandle;

static GLOBAL_HANDLE: Option<JoinHandle<FnOnce(_)>> = None;

fn main() {}
