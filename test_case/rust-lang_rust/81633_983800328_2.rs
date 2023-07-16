
error: cannot find macro `str` in this scope
  --> src/lib.rs:49:13
   |
49 |             str!()
   |             ^^^
   |
   = help: have you added the `#[macro_use]` on the module/import?

warning: unused macro definition: `str`
  --> src/lib.rs:55:18
   |
55 |     macro_rules! str {() => {}}
   |                  ^^^
   |
   = note: `#[warn(unused_macros)]` on by default
