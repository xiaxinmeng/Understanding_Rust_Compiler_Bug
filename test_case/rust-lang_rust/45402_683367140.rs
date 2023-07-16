
[19:17:37] # iximeow:~> rustc -Zpolonius --crate-type cdylib test_polonius2.rs 
warning: struct is never constructed: `Something`
 --> test_polonius2.rs:2:8
  |
2 | struct Something;
  |        ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Struct1`
 --> test_polonius2.rs:3:8
  |
3 | struct Struct1 {
  |        ^^^^^^^

warning: associated function is never used: `get_mut`
 --> test_polonius2.rs:8:5
  |
8 |     fn get_mut(&mut self) -> &mut Something {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted
