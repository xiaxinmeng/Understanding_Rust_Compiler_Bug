patch
   error[E0004]: non-exhaustive patterns: `A(_)` not covered
     --> src/main.rs:12:11
      |
   2  | / enum Foo {
   3  | |     A(bool),
      | |     - not covered
   4  | |     B,
   5  | |     C,
   6  | | }
      | |_- `Foo` defined here
   ...
   12 |       match a {
      |             ^ pattern `A(_)` not covered
+  ...
+  15 |           Foo::A(x) if !x => {},
+     |                     ----- possibly false
      |
      = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
      = note: the matched value is of type `Foo`
