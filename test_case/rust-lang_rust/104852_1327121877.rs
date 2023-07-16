rust
#[cfg_attr(not(windows), inline)]
#[cfg_attr(all(windows, target_thread_local), inline(never))]
