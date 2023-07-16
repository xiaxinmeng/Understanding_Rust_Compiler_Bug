
   Compiling foo v0.1.0 (/Users/mark/Edit/testing/foo)
warning: variant `a` should have an upper camel case name
  --> src/bar.rs:4:11
   |
4  |           a
   |           ^ help: convert the identifier to upper camel case: `A`
   |
  ::: src/lib.rs:10:1
   |
10 | bar!(foo!);
   | ----------- in this macro invocation
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `bbbbb` should have an upper camel case name
  --> src/bar.rs:5:11
   |
5  |           bbbbb
   |           ^^^^^ help: convert the identifier to upper camel case: `Bbbbb`
   |
  ::: src/lib.rs:10:1
   |
10 | bar!(foo!);
   | ----------- in this macro invocation

warning: variant `cccccccccccccccccc` should have an upper camel case name
  --> src/bar.rs:6:11
   |
6  |           cccccccccccccccccc
   |           ^^^^^^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `Cccccccccccccccccc`
   |
  ::: src/lib.rs:10:1
   |
10 | bar!(foo!);
   | ----------- in this macro invocation

    Finished dev [unoptimized + debuginfo] target(s) in 0.25s

