
warning: trait bound Yes: Trait does not depend on any type or lifetime parameters
  --> src/lib.rs:12:30
   |
12 | impl Trait for S1 where Yes: Trait {}
   |                              ^^^^^
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: trait bound No: Trait does not depend on any type or lifetime parameters
  --> src/lib.rs:13:29
   |
13 | impl Trait for S2 where No: Trait {}
   |                             ^^^^^
