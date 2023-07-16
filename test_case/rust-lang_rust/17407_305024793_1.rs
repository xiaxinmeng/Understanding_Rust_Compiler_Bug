
error[E0425]: cannot find value `undefined` in this scope
  --> test.rs:2:25
   |
2  |     () => { fn main() { undefined } };
   |                         ^^^^^^^^^ not found in this scope
...
14 | go!(a 1 2 3 4 5 6 7 8 9 10);
   | ---------------------------- in this macro invocation

error: aborting due to previous error(s)
