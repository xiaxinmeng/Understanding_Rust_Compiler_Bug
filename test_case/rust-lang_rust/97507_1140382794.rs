plain
    |
49  | #[cfg_attr(test, derive(Clone))]
    |                         ----- in this derive macro expansion
...
211 |     pub initial_rustfmt: RefCell<RustfmtState>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `RustfmtState`
    |
    = note: required because of the requirements on the impl of `Clone` for `RefCell<RustfmtState>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
Build completed unsuccessfully in 0:38:30
