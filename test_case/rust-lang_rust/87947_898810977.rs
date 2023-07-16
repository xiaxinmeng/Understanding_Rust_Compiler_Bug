
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyStatus {
    pub func: *const c_char,
    pub err_msg: *const c_char,
    pub exitcode: c_int,
}
