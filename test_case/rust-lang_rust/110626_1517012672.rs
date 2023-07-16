rust
mod read_write {
    pub fn preadv2() {}

    /// [`preadv2`]
    pub use crate::backend::ReadWriteFlags;
}
mod backend {
    pub struct ReadWriteFlags;
}
pub use read_write::{preadv2, ReadWriteFlags};
