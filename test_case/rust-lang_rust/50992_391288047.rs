
warning[E0170]: pattern binding `WebServer` is named the same as one of the variants of the type `Dependency`
  --> src/main.rs:23:9
   |
23 |         WebServer => println!("WebServer"),
   |         ^^^^^^^^^
   |
   = help: if you meant to match on a variant, consider making the path in the pattern qualified: `Dependency::WebServer`

warning[E0170]: pattern binding `LongMemory` is named the same as one of the variants of the type `Dependency`
  --> src/main.rs:24:9
   |
24 |         LongMemory => println!("LongMemory")
   |         ^^^^^^^^^^
   |
   = help: if you meant to match on a variant, consider making the path in the pattern qualified: `Dependency::LongMemory`

warning: unreachable pattern
  --> src/main.rs:24:9
   |
23 |         WebServer => println!("WebServer"),
   |         --------- matches any value
24 |         LongMemory => println!("LongMemory")
   |         ^^^^^^^^^^ unreachable pattern
   |
   = note: #[warn(unreachable_patterns)] on by default
