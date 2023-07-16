rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duk_thread_state {
    pub data: [::std::os::raw::c_char; 128usize],
}
