 Rust
#[deny(warnings)]
#[allow(deprecated)]

fn main() {
    do_stuff();
}

#[deprecated]
fn do_stuff() {}
