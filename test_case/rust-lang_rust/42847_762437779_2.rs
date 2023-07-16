rust
  error[E0271]: type mismatch resolving `<T as Pointee>::Metadata == ()`
     --> library/core/src/sync/atomic.rs:171:24
      |
  171 |         AtomicPtr::new(crate::ptr::null_mut())
      |                        ^^^^^^^^^^^^^^^^^^^^ expected `()`, found associated type
      |
     ::: library/core/src/ptr/mod.rs:227:35
      |
  227 | pub const fn null_mut<T: ?Sized + Thin>() -> *mut T {
      |                                   ---- required by this bound in `ptr::null_mut`
      |
      = note:    expected unit type `()`
              found associated type `<T as Pointee>::Metadata`
      = help: consider constraining the associated type `<T as Pointee>::Metadata` to `()`
      = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced>
  