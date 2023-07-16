 rust
src/source.rs:52:70: 52:79 error: transmute called with differently sized types:
extern "C" fn(&RefCell<Box<FnMut() -> source::Continue + 'static>>) -> i32 {source::trampoline} (0 bits) to
Option<unsafe extern "C" fn(*mut libc::c_void) -> i32> (64 bits) [E0512]                                          
src/source.rs:52         glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, transmute(trampoline),
