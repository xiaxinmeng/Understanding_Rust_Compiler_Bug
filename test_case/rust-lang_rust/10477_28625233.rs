 rust
fn dead_fn() {
}

fn used_fn() {
}

#[main]
fn this_is_main() {
    used_fn();
}

// this is not main
fn main() {
    dead_fn();
}
