 Rust
#[deny(warnings)]
#[warn(deprecated)]

fn main() {
    do_stuff();
}

#[deprecated]
fn do_stuff() {}
