rust
extern crate saltyrtc_client_ffi;

pub type salty_event_loop_t = saltyrtc_client_ffi::salty_event_loop_t;

#[no_mangle]
pub extern "C" fn salty_event_loop_new() -> *mut salty_event_loop_t {
    saltyrtc_client_ffi::salty_event_loop_new()
}
