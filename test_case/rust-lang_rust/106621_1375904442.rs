plain

   Doc-tests rustc_hir

running 11 tests
iiiiiiF....

---- src/def.rs - def::Res::SelfTyAlias::forbid_generic (line 369) stdout ----
---- src/def.rs - def::Res::SelfTyAlias::forbid_generic (line 369) stdout ----
error: cannot use constants which depend on generic parameters in types
  |
  |
4 |     let _bar = [1_u8; std::mem::size_of::<*mut T>()];
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
  = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
  = note: `-D const-evaluatable-unchecked` implied by `-D warnings`
error: aborting due to previous error

Couldn't compile the test.


failures:
    src/def.rs - def::Res::SelfTyAlias::forbid_generic (line 369)

test result: FAILED. 4 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 0.21s

error: doctest failed, to rerun pass `-p rustc_hir --doc`
