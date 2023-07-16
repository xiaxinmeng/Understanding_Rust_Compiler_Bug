
error: Undefined Behavior: `dyn` call trying to call something that is not a method
  --> src/main.rs:14:5
   |
14 |     <dyn X as X>::foo(&()); // Segfault at opt-level 0, SIGILL otherwise.
   |     ^^^^^^^^^^^^^^^^^^^^^^ `dyn` call trying to call something that is not a method
