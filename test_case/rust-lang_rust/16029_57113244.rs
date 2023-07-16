 rust
extern crate syntax;

use syntax::parse::{new_parse_sess};
use std::task;

fn main() {
    task::try(proc() {
        new_parse_sess();
    });
}
