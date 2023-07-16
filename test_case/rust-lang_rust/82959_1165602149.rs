
error[[E0433]](https://doc.rust-lang.org/nightly/error-index.html#E0433): failed to resolve: use of undeclared type `IntoIter`
  --> src/lib.rs:24:24
   |
24 |         let mut iter = IntoIter::new(self);
   |                        ^^^^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
3  | [use arrayvec::IntoIter;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)
   |
3  | [use bit_vec::IntoIter;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)
   |
3  | [use bytes::buf::IntoIter;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)
   |
3  | [use core::array::IntoIter;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018#)
   |
     and 58 other candidates
