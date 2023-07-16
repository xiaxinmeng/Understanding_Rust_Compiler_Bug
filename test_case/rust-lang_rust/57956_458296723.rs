rust
use std::sync::Mutex;
let g = Mutex::new(Mutex::new(String::new()));
let procs = g.lock().unwrap();
let mut q = procs.lock().ok();
let _sp = if let Some(ref _p) = q {
    Some(0)
} else {
    None
};
drop(q);
drop(procs);
