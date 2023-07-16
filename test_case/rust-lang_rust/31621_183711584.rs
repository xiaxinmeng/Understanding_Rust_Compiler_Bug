 Rust
    pub unsafe fn new<F: FnOnce()>(stack: usize, p: F) -> io::Result<Thread> {
        let p = box p;
        match Self::new_inner(stack, &*p as *const F as *const libc::c_void, thread_start::<F>) {
            Ok(o) => { mem::forget(p); Ok(o) }
            Err(e) => Err(e)
        }

        extern fn thread_start<F: FnOnce()>(main: *mut libc::c_void)
            -> *mut libc::c_void {
            unsafe {
                let main = Box::from_raw(main as *mut F);
                start_thread(main);
            }
            ptr::null_mut()
        }
    }

    unsafe fn new_inner<'a>(stack: usize, p: *const libc::c_void,
                            f: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void)
                            -> io::Result<Thread> {
        // ..
    }
