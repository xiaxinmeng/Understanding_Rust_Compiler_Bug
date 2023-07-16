
trait WithCStrs {
    fn with_c_strs<T>(&self, null_terminated: bool, f: &fn(**libc::c_char) -> T) -> T;
}

impl<'self, T: Str> WithCStrs for &'self [T] {
    fn with_c_strs<T: Str, U>(&self, null_terminate: bool, f: &fn(**libc::c_char) -> U) -> U {
        let c_strs: ~[CString] = self.map(|s: & &str| s.to_c_str());
        let mut ptrs: ~[*c_char] = c_strs.map(|c: &CString| c.with_ref(|ptr| ptr));
        if null_terminate {
            ptrs.push(std::ptr::null());
        }
        ptrs.as_imm_buf(|buf: **c_char, _| f(buf))
    }
}
