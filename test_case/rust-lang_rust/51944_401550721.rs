
warning: conflicting implementations of trait `core::convert::Into<core::task::FutureObj<_>>` for type `boxed::PinBox<_>`: (E0119)
   --> liballoc/boxed.rs:953:1
    |
953 | impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for PinBox<F> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[warn(incoherent_fundamental_impls)] on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
    = note: conflicting implementation in crate `core`:
            - impl<T, U> core::convert::Into<U> for T
              where U: core::convert::From<T>;
    = note: downstream crates may implement trait `core::convert::From<boxed::PinBox<_>>` for type `core::task::FutureObj<_>`
