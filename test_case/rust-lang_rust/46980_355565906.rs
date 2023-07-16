
   Compiling cargo v0.25.0 (file:///C:/projects/rust/src/tools/cargo)
error: unnecessary parentheses around method argument
   --> src\tools\cargo\src/cargo\ops\cargo_test.rs:123:52
    |
123 |         Ok((Test::Multiple, errors.into_iter().map((|(_, _, e)| e)).collect()))
    |                                                    ^^^^^^^^^^^^^^^ help: remove these parentheses
    |
note: lint level defined here
   --> src\tools\cargo\src/cargo/lib.rs:1:9
    |
1   | #![deny(unused)]
    |         ^^^^^^
    = note: #[deny(unused_parens)] implied by #[deny(unused)]
error: aborting due to previous error
error: Could not compile `cargo`.
To learn more, run the command again with --verbose.
