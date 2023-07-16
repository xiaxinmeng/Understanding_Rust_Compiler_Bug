rust
// We use a CAS loop to increment the strong count instead of a
// fetch_add because once the count hits 0 it must never be above 0.
let inner = self.inner()?;

// Relaxed load because any write of 0 that we can observe leaves the
// field in a permanently zero state (so a "stale" read of 0 is fine),
// and any other value is confirmed via the CAS.
inner
    .strong
    .fetch_update(
        |n| {
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
        },
        Relaxed,
        Relaxed,
    )
    .ok()
    .map(|_| Arc {
        // null checked above
        ptr: self.ptr,
        phantom: PhantomData,
    })
