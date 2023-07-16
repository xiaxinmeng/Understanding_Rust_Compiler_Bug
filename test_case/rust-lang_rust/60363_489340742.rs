rust
// src/test/ui/issue-60363.rs
// aux-build:issue-60363.rs

extern crate issue_60363;

use issue_60363::Handler;

#[allow(dead_code)]
static HANDLER: Handler = Handler::new();

fn main() {}
