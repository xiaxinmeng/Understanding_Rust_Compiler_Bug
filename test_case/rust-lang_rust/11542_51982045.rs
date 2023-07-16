
    fn pthread_join(native: libc::pthread_t,
                    value: *mut *mut libc::c_void) -> libc::c_int;
