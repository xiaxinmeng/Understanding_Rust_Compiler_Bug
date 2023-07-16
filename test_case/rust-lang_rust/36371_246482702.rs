 bash
error: main function not found

error[E0046]: not all trait items implemented, missing: `fmt`
  --> file4.rs:17:1
   |
17 | impl std::fmt::Display for A {
   | ^ missing `fmt` in implementation

error[E0046]: not all trait items implemented, missing: `Err`, `from_str`
  --> file4.rs:19:1
   |
19 | impl FromStr for A{}
   | ^^^^^^^^^^^^^^^^^^^^ missing `Err`, `from_str` in implementation

error[E0046]: not all trait items implemented, missing: `Foo`, `foo`, `bar`, `bay`
  --> file4.rs:21:1
   |
21 | impl X<usize> for A {
   | ^ missing `Foo`, `foo`, `bar`, `bay` in implementation

error: aborting due to 3 previous errors
