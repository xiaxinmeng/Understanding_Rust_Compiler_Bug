
error: a `where` clause must have at least one predicate in it
  --> /home/nmatsakis/versioned/rust-7/src/test/parse-fail/where-clauses-no-bounds-or-predicates.rs:13:36
   |
13 | fn equal1<T>(_: &T, _: &T) -> bool where {
   |                                    ^^^^^

error: aborting due to previous error
