 rust
extern crate test;

fn abort() -> ! {
    test::black_box(&());
    abort()
}

fn main() {
    abort();
}
