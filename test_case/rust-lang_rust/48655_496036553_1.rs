rust
inner
    .strong
    .fetch_update(Relaxed, Relaxed, |n| {
        if n == 0 {
            return None;
        }

        // See comments in `Arc::clone` for why we do this (for `mem::forget`).
        if n > MAX_REFCOUNT {
            unsafe {
                abort();
            }
        }

        Some(n + 1)
    })
