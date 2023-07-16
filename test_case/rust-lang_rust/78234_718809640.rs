
warning: unreachable pattern
  --> src/main.rs:26:13
   |
26 |             _ => unreachable!("why you do this rust?"),
   |             ^
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: unused variable: `a_res`
  --> src/main.rs:23:9
   |
23 |         a_res = a => match foo {
   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_a_res`
   |
   = note: `#[warn(unused_variables)]` on by default
