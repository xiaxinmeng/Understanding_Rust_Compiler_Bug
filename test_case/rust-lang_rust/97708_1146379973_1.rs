rust
// This is a public type in `maitake`
#[repr(transparent)]
#[cfg_attr(loom, allow(dead_code))]
pub struct TaskStub {
    hdr: Header,
}

impl TaskStub {
    /// Create a new unique stub [`Task`].
    pub const fn new() -> Self {
        Self {
            hdr: Header::new_stub(),
        }
    }
}
