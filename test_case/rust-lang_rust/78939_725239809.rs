rust
// outer
pub mod http {
    //! Various HTTP related types

    // re-exports
    pub use crate::header::HeaderMap;

    /// Various http headers
    pub mod header {
        pub use crate::header::*;
    }
}

mod header {
    pub use iter::HeaderMap;
}

// inner
pub struct HeaderMap {
    pub(crate) inner: std::collections::HashMap<usize, usize>,
}

impl HeaderMap {
    /// Returns a view of all values associated with a key.
    ///
    /// The returned view does not incur any allocations and allows iterating
    /// the values associated with the key.  See [`GetAll`] for more details.
    /// Returns `None` if there are no values associated with the key.
    pub fn get_all(&self) -> GetAll<'_> {
        unimplemented!()
    }
}

pub struct GetAll<'a> {
    idx: usize,
    item: Option<&'a usize>,
}
