plain

   Doc-tests rustc_middle

running 111 tests
iiiiiiiiiiiiii.iii.i.i...F...ii.iii.i..i.i..i.i..i.i..i.i..i.i..i.i..i.i.ii.iiiiii.i.iii  88/111

failures:

---- src/mir/syntax.rs - mir::syntax::BorrowKind::DerefMut (line 237) stdout ----
---- src/mir/syntax.rs - mir::syntax::BorrowKind::DerefMut (line 237) stdout ----
error[E0412]: cannot find type `RefMut` in this scope
 --> src/mir/syntax.rs:238:15
  |
3 | fn bar_mut(y: RefMut<'_, u32>) {
  |
help: consider importing one of these items
  |
2 | use core::cell::RefMut;
---
    src/mir/syntax.rs - mir::syntax::BorrowKind::DerefMut (line 237)

test result: FAILED. 53 passed; 1 failed; 57 ignored; 0 measured; 0 filtered out; finished in 622.26ms

error: doctest failed, to rerun pass `-p rustc_middle --doc`
