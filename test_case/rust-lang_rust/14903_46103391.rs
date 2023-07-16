 rust
pub struct FnPtr<F> { f: F, is_loaded: bool }

impl<F> FnPtr<F> {
    pub fn new(ptr: Option<extern "system" fn()>, failing_fn: F) -> FnPtr<F> {
        use std::mem::transmute;
        match ptr {
            std::option::Some(p) => FnPtr { f: unsafe { transmute(p) }, is_loaded: true },
            None => FnPtr { f: failing_fn, is_loaded: false },
        }
    }
}
