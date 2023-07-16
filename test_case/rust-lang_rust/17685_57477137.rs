 rust
struct Handle<'a> {
    ptr: &'a *mut JSObject,
}

impl<'a> Handle<'a> {
    unsafe fn get(&self) -> *const *mut JSObject {
        self.ptr
    }
}

fn main() {
    unsafe { 
        f(handle.get());
    }
}
