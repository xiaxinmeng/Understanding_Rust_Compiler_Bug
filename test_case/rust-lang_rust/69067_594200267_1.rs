
   Compiling playground v0.0.1 (/playground)
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/lib.rs:9:15
   |
9  |         arg = self;
   |               ^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'_` as defined on the impl at 7:16...
  --> src/lib.rs:7:16
   |
7  | impl Bar for T<'_> {
   |                ^^
note: ...so that the declared lifetime parameter bounds are satisfied
  --> src/lib.rs:9:15
   |
9  |         arg = self;
   |               ^^^^
note: but, the lifetime must be valid for the anonymous lifetime #1 defined on the method body at 8:5...
  --> src/lib.rs:8:5
   |
8  | /     fn bar(self, arg: T) {
9  | |         arg = self;
10 | |     }
   | |_____^
note: ...so that the expression is assignable
  --> src/lib.rs:9:15
   |
9  |         arg = self;
   |               ^^^^
   = note: expected  `std::boxed::Box<dyn Bar>`
              found  `std::boxed::Box<dyn Bar>`

error: aborting due to previous error
