
error: a `where` clause must have at least one predicate in it
  --> /home/nmatsakis/versioned/rust-7/src/test/parse-fail/where-clauses-no-bounds-or-predicates.rs:13:36
   |
13 | fn equal1<T>(_: &T, _: &T) -> bool where {
   |                                    ^^^^^

error: each predicate in a `where` clause must have at least one bound in it
  --> /home/nmatsakis/versioned/rust-7/src/test/parse-fail/where-clauses-no-bounds-or-predicates.rs:18:42
   |
18 | fn equal2<T>(_: &T, _: &T) -> bool where T: {
   |                                          ^^

error: aborting due to previous error
