rust
error: struct literals are not allowed here
  --> src/lib.rs:14:13
   |
14 |     if f == Foo { a: 0, b: 0.0 } {
   |             ^^^^^^^^^^^^^^^^^^^^
   |
help: surround the struct literal with parentheses
   |
14 |     if f == (Foo { a: 0, b: 0.0 }) {
   |             ^                    ^

error: aborting due to previous error
