
error: conflicting implementations of trait `core::convert::Into<core::ptr::NonNull<_>>` for type `boxed::Box<_>`: (E0119)
   --> src/liballoc/boxed.rs:483:1
    |
483 | impl<T: ?Sized> Into<NonNull<T>> for Box<T> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[deny(incoherent_fundamental_impls)] on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
    = note: conflicting implementation in crate `core`:
            - impl<T, U> core::convert::Into<U> for T
              where U: core::convert::From<T>;
    = note: downstream crates may implement trait `core::convert::From<boxed::Box<_>>` for type `core::ptr::NonNull<_>`
