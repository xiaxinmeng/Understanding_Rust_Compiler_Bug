rust
        let mut shared: ThreadMessage = ThreadMessage::None;
        let shared_ptr = std::ptr::addr_of!(shared);
        ...
            scope.spawn(move || {
                ...
                             let work_msg = unsafe { std::ptr::read(shared_ptr) };
