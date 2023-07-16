rust
error[E0271]: type mismatch resolving `<T as Pointee>::Metadata == ()`
   --> library/core/src/sync/atomic.rs:177:24
    |
177 |         AtomicPtr::new(crate::ptr::null_mut())
    |                        ^^^^^^^^^^^^^^^^^^^^ expected `()`, found associated type
    |
    = note:    expected unit type `()`
            found associated type `<T as Pointee>::Metadata`
    = help: consider constraining the associated type `<T as Pointee>::Metadata` to `()`
    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `ptr::null_mut`
   --> library/core/src/ptr/mod.rs:232:35
    |
232 | pub const fn null_mut<T: ?Sized + Thin>() -> *mut T {
    |                                   ^^^^ required by this bound in `ptr::null_mut`
