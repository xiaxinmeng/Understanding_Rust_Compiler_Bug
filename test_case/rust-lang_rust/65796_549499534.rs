rust
                    let mut node = Waiter {
                        thread: Some(thread::current()),
                        signaled: AtomicBool::new(false),
                        next: ptr::null_mut(),
                    };
                    let node = &mut node as *mut Waiter; // henceforth only use this
