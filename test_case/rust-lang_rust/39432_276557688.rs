rust
fn worker() -> ! {
    panic!()
}

fn main() {
    std::panic::catch_unwind(worker);
}
