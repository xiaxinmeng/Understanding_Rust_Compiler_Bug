
error: proc macro panicked
 --> src/main.rs:3:1
  |
3 | / speculate! {
4 | |     describe hello {
5 | |         it "panics" {
6 | |             assert!(true);
7 | |         }
8 | |     }
9 | | }
  | |_^
  |
  = help: message: called `Result::unwrap()` on an `Err` value: ParseError(Some("failed to parse anything")
