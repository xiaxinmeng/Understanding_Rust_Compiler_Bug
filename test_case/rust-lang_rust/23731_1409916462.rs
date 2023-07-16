rust
#[cfg(feature = "new-pathbuf-pop")]
pub fn pop(&mut self) -> Option<OsStr> { .. }

#[cfg(not(feature = "new-pathbuf-pop"))]
#[cfg_attr(
    not(feature = "new-pathbuf-pop"),
    deprecated = "We recommend you to enable \"new-pathbuf-pop\" feature and try new pop() method, or existing parent() method instead.")]
pub fn pop(&mut self) -> bool { .. }
