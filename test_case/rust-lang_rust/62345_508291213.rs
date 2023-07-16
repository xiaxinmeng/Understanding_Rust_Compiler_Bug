rust
/! This is an inline link to [`CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html).
//! It works!
//!
//! This is an intra-crate footnote link to [`MyType`][MyType].  It also works.
//!
//! This is a footnote link, aka "reference link", to [`OsStr`][OsStr].  It work
s.
//!
//! This is an implied footnote link to [`CString`].  It fails :( .
//!
//! This is an implied link that can't be an identifier [`some-thing`].  It fails, but doesn't
//! print a warning.
//!
//! [MyType]: t/struct.MyType.html
//! [OsStr]: https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html
//! [CString]: https://doc.rust-lang.org/stable/std/ffi/struct.CString.html
//! [some-thing]: http://some-thing.com
pub mod t {
    pub struct MyType {}
}
