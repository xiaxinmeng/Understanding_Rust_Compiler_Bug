rust
let _guard = acquire_env_lock();
let old_env = *sys::os::environ();
*sys::os::environ() = new_env;
let _reset_on_drop = on_drop(|| { *sys::os::environ() = old_env; });

// do the exec
