 rust
extern crate test;

fn abort() -> ! {
    let xs = Vec::from_elem(std::uint::MAX, 1u8);
    test::black_box(&xs);
    loop {}
}

fn main() {
    abort();
}
