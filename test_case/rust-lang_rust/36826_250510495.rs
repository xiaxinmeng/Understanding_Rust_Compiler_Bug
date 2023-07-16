
#[cfg(target_os = "linux")]
unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern fn(*mut u8)) {

    extern {
        #[linkage = "extern_weak"]
        static __dso_handle: *mut u8;
    }

    __cxa_thread_atexit_impl(dtor, t, &__dso_handle as *const _ as *mut _);
}

#[cfg(target_os = "linux")]
#[linkage = "weak"]
extern "C" unsafe fn __cxa_thread_atexit_impl(dtor: unsafe extern fn(*mut u8),
                                  t: *mut u8,
                                  dso_handle: *mut u8) -> libc::c_int {

    use mem;
    use libc;
    use sys_common::thread_local as os;

    // The fallback implementation uses a vanilla OS-based TLS key to track
    // the list of destructors that need to be run for this thread. The key
    // then has its own destructor which runs all the other destructors.
    //
    // The destructor for DTORS is a little special in that it has a `while`
    // loop to continuously drain the list of registered destructors. It
    // *should* be the case that this loop always terminates because we
    // provide the guarantee that a TLS key cannot be set after it is
    // flagged for destruction.
    static DTORS: os::StaticKey = os::StaticKey::new(Some(run_dtors));
    type List = Vec<(*mut u8, unsafe extern fn(*mut u8))>;
    if DTORS.get().is_null() {
        let v: Box<List> = box Vec::new();
        DTORS.set(Box::into_raw(v) as *mut u8);
    }
    let list: &mut List = &mut *(DTORS.get() as *mut List);
    list.push((t, dtor));

    unsafe extern fn run_dtors(mut ptr: *mut u8) {
        while !ptr.is_null() {
            let list: Box<List> = Box::from_raw(ptr as *mut List);
            for &(ptr, dtor) in list.iter() {
                dtor(ptr);
            }
            ptr = DTORS.get();
            DTORS.set(ptr::null_mut());
        }
    }
}
