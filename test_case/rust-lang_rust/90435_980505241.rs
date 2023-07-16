rust
struct ffi_foo;

#[repr(transparent)]
pub struct Foo {
    inner: UnsafeCell<ffi_foo>,
}

impl Foo {
    pub fn go_kazoo(&self) {
        unsafe { ffi_foo_go_kazoo(self.inner.get()) }
    }
}

impl<'a> From<&'a ffi_foo> for &'a Foo {
    fn from(x: &'a ffi_foo) -> Self {
        // SAFETY: types have identical layout and ffi_foo is thread-safe
        unsafe { std::mem::transmute(x) }
    }
}
