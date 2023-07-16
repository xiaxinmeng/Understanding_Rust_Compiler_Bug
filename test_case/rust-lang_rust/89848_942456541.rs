rust
use std::ptr::NonNull;

#[derive(Clone)]
pub struct A(NonNull<()>);

impl Drop for A {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl A {
    pub fn bug(&self) -> &() {
        unsafe { self.clone().0.as_ref() }
    }
}
