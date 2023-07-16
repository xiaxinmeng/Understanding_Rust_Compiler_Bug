rust
        // If we successfully register an at exit handler, then we cache the
        // `Arc` allocation in our own internal box (it will get deallocated by
        // the at exit handler). Otherwise we just return the freshly allocated
        // `Arc`.
        let registered = sys_common::at_exit(move || {
            let ptr = {
                let _guard = self.lock.lock();
                self.ptr.replace(done())
            };
            drop(Box::from_raw(ptr))
        });
