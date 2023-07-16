
warning: unused `Foo` that must be used
  --> src/lib.rs:8:5
   |
8  | /     match f {
9  | |         Foos::F(f) => f
10 | |     };
   | |______^
   |
   = note: `#[warn(unused_must_use)]` on by default
