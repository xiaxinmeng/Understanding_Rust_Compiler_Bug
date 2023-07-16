
error: multiple unused formatting arguments
  -->  /Users/nickvernij/Programming/rust/src/test/ui/macros/format-foreign.rs:12:5
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^--------^^-------^^-^^
   |                              |         |        |
   |                              |         |        unused
   |                              |         unused
   |                              unused
   |
