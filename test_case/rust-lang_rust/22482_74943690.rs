 rust
let ptr = opt_cstr.map(|s| s.as_ptr()).unwrap_or(0 as *const _);
