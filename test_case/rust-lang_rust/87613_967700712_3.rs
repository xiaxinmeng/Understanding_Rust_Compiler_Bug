
% cargo +nightly build
   Compiling issue_87613 v0.1.0 (/home/ubuntu/Dev/Rust/rust-87613/objdir-dbg/issue_87613)
error[E0433]: failed to resolve: use of undeclared type `Command`
 --> src/main.rs:4:5
  |
4 |     Command::new("git");
  |     ^^^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
3 | hey */ use std::process::Command;
  |        ++++++++++++++++++++++++++
3 | hey */ use tokio::process::Command;
  |        ++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0433`.
error: could not compile `issue_87613` due to previous error
%
