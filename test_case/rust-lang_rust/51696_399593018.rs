
error[E0308]: mismatched types
  --> ../../src/test/compile-fail/issue-37665.rs:20:17
   |
20 |     let x: () = 0; //~ ERROR: mismatched types
   |                 ^ expected (), found integral variable
   |
   = note: expected type `()`
              found type `{integer}`

warning: variable does not need to be mutable
  --> ../../src/test/compile-fail/issue-37665.rs:17:9
   |
17 |     let mut foo : String = "hello".to_string();
   |         ----^^^
   |         |
   |         help: remove this `mut`
   |
   = note: #[warn(unused_mut)] on by default
