 rust
// now
pub fn load_with(loadfn: |symbol: &str| -> Option<extern "system" fn()>) {
    unsafe { ::storage::$name = ::FnPtr::new(loadfn($sym), ::failing::$name) }
}
// after
pub fn load_with(loadfn: |symbol: &str| -> Option<extern "system" fn()>) {
    match loadfn($sym) {
        std::option::Some(f) => unsafe {
            ::storage::$name = ::FnPtr { f: std::mem::transmute(f), is_loaded: true }
        },
        None => {}
    }
}
