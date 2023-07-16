
error: Undefined Behavior: trying to reborrow for SharedReadWrite, but parent tag <50139> does not have an appropriate item in the borrow stack
  --> btree.rs:11:26
   |
11 |         mem::swap(dummy, r);
   |                          ^ trying to reborrow for SharedReadWrite, but parent tag <50139> does not have an appropriate item in the borrow stack
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
