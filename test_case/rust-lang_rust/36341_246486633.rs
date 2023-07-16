 rust
struct ThreadId(libc::pthread_t);

impl PartialEq for ThreadId { ... }
