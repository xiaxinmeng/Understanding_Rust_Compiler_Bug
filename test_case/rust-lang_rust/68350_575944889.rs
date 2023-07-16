rust
fn produce_it<F: FnOnce() -> R, R>(val: F) -> R {
    val()
}

fn main() {
    produce_it(|| panic!());
}
