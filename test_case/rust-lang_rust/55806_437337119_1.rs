rust
extern crate issue_55806_lib;

use issue_55806_lib::bar;

fn main() {
    let _x: &'static _ = &bar();
}
