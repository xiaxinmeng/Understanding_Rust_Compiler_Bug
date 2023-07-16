 rust
> cargo new foobar
> cd foobar

// in src/lib.rs
mod foo {
    mod sub {
        pub struct Test(i32);
    }
}

use foo::sub::Test;
