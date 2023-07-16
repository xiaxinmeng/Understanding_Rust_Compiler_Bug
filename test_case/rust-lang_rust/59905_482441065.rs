
pub struct CStr {
    inner: [c_char; 1],
    _marker: PhantomUnsized
}
