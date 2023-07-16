 rust
            #[cfg(target_os = "macos")]
            extern {
                fn _tlv_atexit(dtor: unsafe extern "C" fn(ptr: *mut c_void), ptr: *mut c_void);
            }

            #[cfg(target_os = "linux")]
            extern {
                static mut __dso_handle: i8;
                fn __cxa_thread_atexit_impl(dtor: unsafe extern "C" fn(ptr: *mut c_void), ptr: *mut c_void,
                                            dso_symbol: *mut i8);
            }

            #[thread_local]
            pub static mut PTR: *mut $t = 0 as *mut $t;

            unsafe extern "C" fn destructor(ptr: *mut c_void) {
                ::std::ptr::read(ptr as *const $t);
            }

            #[inline(always)]
            #[cfg(not(target_os = "macos"), not(target_os = "linux"))]
            unsafe fn register_destructor() {
                ::std::intrinsics::abort(); // TODO: not yet implemented
            }

            #[cfg(target_os = "linux")]
            unsafe fn register_destructor() {
                __cxa_thread_atexit_impl(destructor, PTR as *mut c_void, &mut __dso_handle);
            }

            #[cfg(target_os = "macos")]
            unsafe fn register_destructor() {
                _tlv_atexit(destructor, PTR as *mut c_void);
            }

            #[inline(always)]
            fn init() {
                unsafe {
                    if PTR.is_null() {
                        PTR = ::std::rt::heap::allocate(::std::mem::size_of::<$t>(),
                                                          ::std::mem::align_of::<$t>()) as *mut $t;
                        *PTR = $init;
                        if ::std::intrinsics::needs_drop::<$t>() {
                            register_destructor();
                        }
                    }
                }
            }
