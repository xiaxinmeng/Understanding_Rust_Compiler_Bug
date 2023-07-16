rust
// self: CString, which is just a newtype around Box<[u8]>.
fn into_inner(self) -> Box<[u8]> {
    unsafe {
        mem::transmute(self)
    }
}
