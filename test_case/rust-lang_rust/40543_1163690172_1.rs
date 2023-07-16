
warning: constant `SOME_FOO` contains interior mutability which cannot be observed
 --> src/lib.rs:2:11
  |
2 | pub const SOME_FOO: Cell<u32> = Cell::new(10);
  |     ^^^^^ help: use a `static` or `static mut` instead
  |
  = note: `#[warn(interior_mutable_const)]` on by default

