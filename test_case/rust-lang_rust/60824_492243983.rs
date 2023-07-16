
warning: unreachable pattern
  --> src/main.rs:10:13
   |
9  |             A => String::from("A"),
   |             - matches any value
10 |             B => String::from("B"),
   |             ^ unreachable pattern
   |
   = note: #[warn(unreachable_patterns)] on by default

warning: unreachable pattern
  --> src/main.rs:11:13
   |
9  |             A => String::from("A"),
   |             - matches any value
10 |             B => String::from("B"),
11 |             C => String::from("C"),
   |             ^ unreachable pattern

warning: unused variable: `A`
 --> src/main.rs:9:13
  |
9 |             A => String::from("A"),
  |             ^ help: consider prefixing with an underscore: `_A`
  |
  = note: #[warn(unused_variables)] on by default

warning: unused variable: `B`
  --> src/main.rs:10:13
   |
10 |             B => String::from("B"),
   |             ^ help: consider prefixing with an underscore: `_B`

warning: unused variable: `C`
  --> src/main.rs:11:13
   |
11 |             C => String::from("C"),
   |             ^ help: consider prefixing with an underscore: `_C`

warning: variable `A` should have a snake case name
 --> src/main.rs:9:13
  |
9 |             A => String::from("A"),
  |             ^ help: convert the identifier to snake case: `a`
  |
  = note: #[warn(non_snake_case)] on by default

warning: variable `B` should have a snake case name
  --> src/main.rs:10:13
   |
10 |             B => String::from("B"),
   |             ^ help: convert the identifier to snake case: `b`

warning: variable `C` should have a snake case name
  --> src/main.rs:11:13
   |
11 |             C => String::from("C"),
   |             ^ help: convert the identifier to snake case: `c`
