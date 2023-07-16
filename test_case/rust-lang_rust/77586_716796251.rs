rust
mod foo {
    pub mod bar {
        pub mod bar {
            pub fn foobar() {}
        }
    }
}

use foo::*;
use bar::bar;
use bar::foobar;

fn main() {
    bar::foobar();
}
