Rust
type Inner = Box<i32>;
struct Wrapper(Inner);

fn main() {
    Wrapper(Box::new(42));
}
