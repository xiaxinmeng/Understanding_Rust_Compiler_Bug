rust
mod foo {
    pub mod bar {
        pub mod bar {
            pub fn foobar() { println!("111") }
        }
        pub fn foobar() { println!("222") }
    }
}

use foo::*;
use bar::bar;
use bar::foobar;

fn main() {
    foobar();
}
