
warning: trait bound Yes: Trait does not depend on any type or lifetime parameters
  --> src/lib.rs:12:30
   |
12 | impl Trait for S1 where Yes: Trait {}
   |                              ^^^^^
   |
   = note: this bound always holds
   = note: `#[warn(trivially_true_bounds)]` on by default

error: trait bound No: Trait does not depend on any type or lifetime parameters
  --> src/lib.rs:13:29
   |
13 | impl Trait for S2 where No: Trait {}
   |                             ^^^^^
   |
   = note: this bound never holds
   = note: `#[error(trivially_false_bounds)]` on by default
