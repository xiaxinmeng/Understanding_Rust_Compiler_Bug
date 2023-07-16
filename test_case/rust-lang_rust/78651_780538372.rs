bash
$ cargo bisect-rustc --test-dir=. --end=ee88f46bb5e27c4d9f30326e69ff2298dcbeecb1 --regress=ice
...
ERROR: the commit at the end of the range (ee88f46bb5e27c4d9f30326e69ff2298dcbeecb1) does not reproduce the regression
$ cargo build
   Compiling bisect v0.1.0 (/home/habbasi/bisect)
error[E0573]: expected type, found module `result`
   --> src/main.rs:2:6
    |
2   |   impl result {
    |        ^^^^^^ help: an enum with a similar name exists: `Result`

error[E0573]: expected type, found variant `Err`
 --> src/main.rs:3:25
  |
3 |     fn into_future() -> Err {}
  |                         ^^^ not a type
  |
help: try using the variant's enum
  |
3 |     fn into_future() -> core::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^^
3 |     fn into_future() -> crate::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^^^
3 |     fn into_future() -> std::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^

error[E0601]: `main` function not found in crate `bisect`
 --> src/main.rs:1:1
  |
1 | / use std::result;
2 | | impl result {
3 | |     fn into_future() -> Err {}
4 | | }
  | |_^ consider adding a `main` function to `src/main.rs`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0573, E0601.
For more information about an error, try `rustc --explain E0573`.
error: could not compile `bisect`

To learn more, run the command again with --verbose.
