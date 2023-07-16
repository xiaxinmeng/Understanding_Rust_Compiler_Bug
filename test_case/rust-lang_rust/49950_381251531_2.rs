
error[E0391]: cyclic dependency detected
 --> e.rs:5:1
  |
5 | trait C: B {}
  | ^^^^^^^^^^ cyclic reference
  |
note: the cycle begins when computing the supertraits of `B`...
 --> e.rs:1:1
  |
1 | trait A: B {}
  | ^^^^^^^^^^
note: ...which then requires computing the supertraits of `C`...
 --> e.rs:3:1
  |
3 | trait B: C {}
  | ^^^^^^^^^^
  = note: ...which then again requires computing the supertraits of `B`, completing the cycle.
