
mod intrinsics {
    extern {
        fn get(&self) -> T;
        fn set(&mut self, value: T);
        fn memcpy<T>(dest: &mut RawPtr<T>, src: &RawPtr<T>, len: uint);
        ...
    }
}

struct RawPtr<T>;

impl<T> RawPtr<T> {
    unsafe fn get(&self) -> T { intrinsics::ptr_get(self) }
    unsafe fn set(&mut self, value: T) { intrinsics::ptr_set(self, value) }
    fn is_null(&self) -> bool {
        unsafe {
            let x: uint = cast::intrinsics(self);
            x == 0
        }
    }
    fn offset<'a>(&'a self, len: uint) -> &'a RawPtr<T> { ... }
    ...
}

fn as_imm_ptr<'a, T>(v: &'a [T]) -> &'a RawPtr<T> {
    let (ptr, _): (&'a RawPtr<T>, uint) = cast::transmute(v);
    ptr
}

mod libc {
    extern {
        fn getenv(&RawPtr<libc::c_char>) -> &'static RawPtr<libc::c_char>;
}

fn getenv(s: &str) -> Option<~str> {
    unsafe {
        let charp = s.as_c_str(|ptr| libc::getenv(ptr));
        if charp.is_null() {
            None
        } else {
            str::from_c_str(charp)
        }
    }
}

...
