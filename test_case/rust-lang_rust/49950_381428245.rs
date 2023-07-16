
error[E0391]: cycle detected when computing the supertraits of `B`
 --> e.rs:3:1
  |
3 | trait B: C {}
  | ^^^^^^^^^^
  |
note: ...which requires computing the supertraits of `C`...
 --> e.rs:5:1
  |
5 | trait C: B {}
  | ^^^^^^^^^^
  = note: ...which again requires computing the supertraits of `B`, completing the cycle
note: cycle used when computing the supertraits of `A`
 --> e.rs:1:1
  |
1 | trait A: B {}
  | ^^^^^^^^^^
