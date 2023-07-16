rust
use std::os::unix::thread::*;

extern "C" {
    pub fn pthread_cancel(h: RawPthread) -> i32;
    pub fn pthread_create(t: *mut RawPthread, a: *mut (), t: extern "C" fn(*const ()) -> *const (), b: *mut ()) -> i32;
}

extern "C" fn banana(x: *const ()) -> *const () {
    ::std::thread::sleep(::std::time::Duration::from_millis(500)); // might unwind due to pthread_cancel and is UB
    x
}

fn main() {
    unsafe {
        let mut t: RawPthread = ::std::mem::zeroed();
        pthread_create(&mut t, ::std::ptr::null_mut(), banana, ::std::ptr::null_mut());
        pthread_cancel(t);
    }
    ::std::thread::sleep(::std::time::Duration::from_millis(1000));
}
