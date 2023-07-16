
error[E0004]: non-exhaustive patterns: multiple patterns of type `MyEnum` are not handled
 --> src/main.rs:7:11
  |
1 | / enum MyEnum {
2 | |     A,
  | |     - variant not covered
3 | |     B,
  | |     - variant not covered
4 | | }
  | |_- `MyEnum` defined here
...
7 |       match MyEnum::A {}
  |             ^^^^^^^^^
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arm
