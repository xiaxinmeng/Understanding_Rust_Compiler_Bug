
error[E0308]: mismatched types
  --> ../../src/test/ui/block-result/unexpected-return-on-unit.rs:19:5
   |
19 |     foo()
   |     ^^^^^ expected (), found usize
   |
   = note: expected type `()`
              found type `usize`
help: did you mean to add a semicolon here?
   |
19 |     foo();
   |          ^
help: possibly return type missing here?
   |
18 | fn bar() -> usize {
   |          ^^^^^^^^
error: aborting due to previous error(s)
