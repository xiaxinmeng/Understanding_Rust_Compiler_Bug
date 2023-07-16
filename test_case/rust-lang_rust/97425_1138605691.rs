plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0119]: conflicting implementations of trait `core::convert::From<rc::Rc<str>>` for type `rc::Rc<[u8]>`
     |
     |
1900 | impl From<Rc<str>> for Rc<[u8]> {
     | ------------------------------- first implementation here
...
1983 | impl From<Rc<str>> for Rc<[u8]> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `rc::Rc<[u8]>`

error[E0119]: conflicting implementations of trait `core::convert::From<sync::Arc<str>>` for type `sync::Arc<[u8]>`
     |
     |
2501 | impl From<Arc<str>> for Arc<[u8]> {
     | --------------------------------- first implementation here
...
2584 | impl From<Arc<str>> for Arc<[u8]> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `sync::Arc<[u8]>`
error[E0711]: feature `shared_from_str` is declared stable since 1.62.0, but was previously declared stable since 1.63.0
    --> library/alloc/src/rc.rs:1982:1
     |
     |
1982 | #[stable(feature = "shared_from_str", since = "1.62.0")]

error[E0711]: feature `shared_from_str` is declared stable since 1.62.0, but was previously declared stable since 1.63.0
    --> library/alloc/src/sync.rs:2583:1
     |
     |
2583 | #[stable(feature = "shared_from_str", since = "1.62.0")]

For more information about this error, try `rustc --explain E0119`.
error: could not compile `alloc` due to 4 previous errors
Build completed unsuccessfully in 0:01:39
