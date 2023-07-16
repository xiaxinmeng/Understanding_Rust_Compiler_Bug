rust
mod foo {
    pub mod bar {
        pub fn foobar() -> u32 { 1 }
        pub mod bar {
            pub fn foobar() -> u32 { 2 }
        }
    }
}

use foo:*;
use bar::bar;
use bar::foobar;
